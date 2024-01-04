//! GeoJSON sink

use std::fs::File;
use std::io::{BufWriter, Write};

use rayon::prelude::*;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use nusamai_citygml::object::CityObject;
use nusamai_geojson::conversion::{
    multilinestring_to_geojson_geometry, multipoint_to_geojson_geometry,
    multipolygon_to_geojson_geometry,
};

pub struct GeoJsonSinkProvider {}

impl DataSinkProvider for GeoJsonSinkProvider {
    fn create(&self, _config: &Config) -> Box<dyn DataSink> {
        Box::<GeoJsonSink>::default()
    }

    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "GeoJSON Sink".to_string(),
        }
    }

    fn config(&self) -> Config {
        Config::default()
    }
}

#[derive(Default)]
pub struct GeoJsonSink {
    n_features: usize,
}

impl DataSink for GeoJsonSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(100);

        rayon::join(
            || {
                // Convert CityObjects to GeoJSON objects

                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        let features = toplevel_cityobj_to_geojson_features(&parcel.cityobj);

                        if sender.send(features).is_err() {
                            println!("sink cancelled");
                            return Err(());
                        };

                        Ok(())
                    },
                );
            },
            || {
                // Write GeoJSON to a file

                // TODO: Handle output file path
                let mut file = File::create("output.geojson").unwrap();
                let mut writer = BufWriter::new(&mut file);

                // Write the FeatureCollection header
                writer
                    .write_all(b"{\"type\":\"FeatureCollection\",\"features\":[")
                    .unwrap();

                // Write each Feature
                let mut iter = receiver.into_iter().flatten().peekable();
                while let Some(feat) = iter.next() {
                    serde_json::to_writer(&mut writer, &feat).unwrap();
                    if iter.peek().is_some() {
                        writer.write_all(b",").unwrap();
                    };
                }

                // Write the FeautureCollection footer and EOL
                writer.write_all(b"]}\n").unwrap();

                println!("Wrote {} features", self.n_features);
            },
        );
    }
}

fn extract_properties(tree: &nusamai_citygml::object::Value) -> Option<geojson::JsonObject> {
    match &tree {
        feat @ nusamai_citygml::Value::Feature(_) => match feat.to_attribute_json() {
            serde_json::Value::Object(map) => Some(map),
            _ => unreachable!(),
        },
        _ => panic!("Root value type must be Feature, but found {:?}", tree),
    }
}

/// Create GeoJSON features from a TopLevelCityObject
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
// TODO: Handle properties (`obj.root` -> `geojson::Feature.properties`)
// TODO: We may want to traverse the tree and create features for each semantic child in the future
pub fn toplevel_cityobj_to_geojson_features(obj: &CityObject) -> Vec<geojson::Feature> {
    let mut geojson_features: Vec<geojson::Feature> = vec![];
    let properties = extract_properties(&obj.root);

    if !obj.geometries.multipolygon.is_empty() {
        let mpoly_geojson_geom = multipolygon_to_geojson_geometry(
            &obj.geometries.vertices,
            &obj.geometries.multipolygon,
        );

        let mpoly_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoly_geojson_geom),
            id: None,
            properties: properties.clone(),
            foreign_members: None,
        };
        geojson_features.push(mpoly_geojson_feat);
    }

    if !obj.geometries.multilinestring.is_empty() {
        let mls_geojson_geom = multilinestring_to_geojson_geometry(
            &obj.geometries.vertices,
            &obj.geometries.multilinestring,
        );
        let mls_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mls_geojson_geom),
            id: None,
            properties: properties.clone(),
            foreign_members: None,
        };
        geojson_features.push(mls_geojson_feat);
    }

    if !obj.geometries.multipoint.is_empty() {
        let mpoint_geojson_geom =
            multipoint_to_geojson_geometry(&obj.geometries.vertices, &obj.geometries.multipoint);
        let mpoint_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoint_geojson_geom),
            id: None,
            properties,
            foreign_members: None,
        };
        geojson_features.push(mpoint_geojson_feat);
    }

    geojson_features
}

#[cfg(test)]
mod tests {
    use super::*;
    use nusamai_citygml::{object::Feature, Value};
    use nusamai_geometry::MultiPolygon;

    #[test]
    fn test_toplevel_cityobj_multipolygon() {
        let vertices: Vec<[f64; 3]> = vec![
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
        ];
        let mut mpoly = MultiPolygon::<'_, 1, u32>::new();
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        let geometries = nusamai_citygml::Geometries {
            vertices,
            multipolygon: mpoly,
            multilinestring: Default::default(),
            multipoint: Default::default(),
        };

        let obj = CityObject {
            root: Value::Feature(Feature {
                typename: "dummy".into(),
                id: None,
                attributes: Default::default(),
                geometries: None,
            }),
            geometries,
        };

        let geojson_features = toplevel_cityobj_to_geojson_features(&obj);
        assert_eq!(geojson_features.len(), 1);

        let mpoly_geojson = geojson_features.first().unwrap();
        assert!(mpoly_geojson.bbox.is_none());
        assert!(mpoly_geojson.foreign_members.is_none());
        if let geojson::Value::MultiPolygon(rings_list) =
            mpoly_geojson.geometry.clone().unwrap().value
        {
            for (i, rings) in rings_list.iter().enumerate() {
                match i {
                    0 => {
                        assert_eq!(rings.len(), 1);
                        assert_eq!(
                            rings[0],
                            vec![
                                [0., 0., 111.],
                                [5., 0., 111.],
                                [5., 5., 111.],
                                [0., 5., 111.],
                                [0., 0., 111.]
                            ]
                        );
                    }
                    _ => unreachable!("Unexpected number of polygons"),
                }
            }
        } else {
            unreachable!("The result is not a GeoJSON MultiPolygon");
        };
    }
}