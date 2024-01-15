pub mod common;

use common::load_cityobjs;
use common::load_cityobjs_from_zstd;
use nusamai_citygml::{Code, Date, Measure};
use nusamai_plateau::models::relief;
use nusamai_plateau::models::uro;
use nusamai_plateau::models::TopLevelCityObject;

#[test]
fn load_area_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/area/523846_area_6697.gml");
    assert_eq!(cityobjs.len(), 4);
    let TopLevelCityObject::Zone(zone) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Zone");
    };

    assert_eq!(
        zone.function,
        vec![Code::new("港湾区域".into(), "0201".into())]
    );
    assert_eq!(zone.urf_valid_from, Date::from_ymd_opt(1, 1, 1));
    assert_eq!(
        zone.valid_from_type,
        Code::new("決定".into(), "1".into()).into()
    );
}

#[test]
fn load_bridge_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/brid/51324378_brid_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::Bridge(bridge) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a CityFurniture");
    };

    assert_eq!(
        bridge.class,
        Some(Code::new("アーチ橋".to_string(), "03".to_string()))
    );
    assert_eq!(
        bridge.function,
        vec![Code::new("道路橋".to_string(), "01".to_string())]
    );
    assert_eq!(bridge.year_of_construction, Some("1962".to_string()));
    assert_eq!(bridge.is_movable, Some(false));
    assert_eq!(
        bridge.outer_bridge_construction[0].function,
        vec![Code::new("アーチ".to_string(), "04".to_string())]
    )
}

#[test]
fn load_building_lod4_example() {
    let cityobjs = load_cityobjs_from_zstd(
        "./tests/data/tokyo23-ku/udx/bldg/53393680_bldg_6697_lod4.2_op.gml.zst",
    );

    let mut multipolygons = 0;
    let mut buildings = 0;
    let mut cityobjectgroups = 0;

    assert_eq!(cityobjs.len(), 1527);

    for cityobj in cityobjs {
        multipolygons += cityobj.geometries.multipolygon.len();
        match cityobj.cityobj {
            TopLevelCityObject::Building(_building) => {
                buildings += 1;
            }
            TopLevelCityObject::CityObjectGroup(_group) => {
                cityobjectgroups += 1;
            }
            _ => {}
        }
    }

    assert_eq!(buildings, 1485);
    assert_eq!(cityobjectgroups, 42);
    assert_eq!(multipolygons, 197633);
}

#[test]
fn load_cityfurniture_example() {
    let cityobjs = load_cityobjs("./tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml");
    assert_eq!(cityobjs.len(), 28);
    let TopLevelCityObject::CityFurniture(frn) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a CityFurniture");
    };

    assert_eq!(frn.function, vec![Code::new("柱".into(), "4800".into())]);
    assert_eq!(
        frn.city_furniture_data_quality_attribute
            .as_ref()
            .unwrap()
            .src_scale,
        vec![Code::new("地図情報レベル500".into(), "3".into(),)]
    );
}

#[test]
fn load_generics_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/gen/53392565_gen_6697.gml");
    assert_eq!(cityobjs.len(), 4);
    let TopLevelCityObject::GenericCityObject(_gen) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a GenericCityObject");
    };
}

#[test]
fn load_landslide_example() {
    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/lsld/523857_lsld_6668_op.gml");
    assert_eq!(cityobjs.len(), 81);
    let TopLevelCityObject::SedimentDisasterProneArea(lsld) = &cityobjs.first().unwrap().cityobj
    else {
        panic!("expected SedimentDisasterProneArea");
    };
    assert_eq!(lsld.location, Some("沼津市下香貫八重".into()));
    assert_eq!(lsld.disaster_type.as_ref().unwrap().code(), "1");
    assert_eq!(lsld.area_type.as_ref().unwrap().code(), "2");
    assert_eq!(lsld.zone_number.as_ref().unwrap(), "103-Ⅰ-0648");
    assert_eq!(lsld.status.as_ref().unwrap().code(), "0");
}

#[test]
fn load_landuse_example() {
    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/luse/523836_luse_6668_op.gml");
    assert_eq!(cityobjs.len(), 225);
    let TopLevelCityObject::LandUse(landuse) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Landuse");
    };

    assert_eq!(
        landuse.land_use_detail_attribute[0].prefecture,
        Some(Code::new("静岡県".into(), "22".into()))
    );
    assert_eq!(
        landuse.land_use_detail_attribute[0].city,
        Some(Code::new("静岡県沼津市".into(), "22203".into()))
    );
}

#[test]
fn load_other_construction_example() {
    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384697_cons_6697.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384698_cons_6697_1.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/52384698_cons_6697_2.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/53394695_cons_6697.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/53395603_cons_6697.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/cons/56403133_cons_6697.gml");
        assert_eq!(cityobjs.len(), 1);
        let TopLevelCityObject::OtherConstruction(cons) = &cityobjs.first().unwrap().cityobj else {
            panic!("must be OtherConstruction");
        };
        let uro::DmAttributeProperty::DmGeometricAttribute(_) = cons.cons_dm_attribute[0] else {
            panic!("must be DmGeometricAttribute");
        };
        assert_eq!(
            cons.cons_base_attribute.as_ref().unwrap().admin_type,
            Some(Code::new("北陸地方整備局".into(), "23".into()))
        );
        assert_eq!(
            cons.cons_base_attribute.as_ref().unwrap().administorator,
            Some("信濃川河川事務所".into())
        )
    }
}

#[test]
fn load_dem_example() {
    let cityobjs = load_cityobjs("./tests/data/tokyo23-ku/udx/dem/533937_dem_6697_op.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::ReliefFeature(dem) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a ReliefFeature");
    };

    let relief::ReliefComponentProperty::TINRelief(tin) = &dem.relief_component[0] else {
        panic!("Unexpected relief component type");
    };
    assert_eq!(tin.lod, Some(1));
}

#[test]
fn load_road_example() {
    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/tran/52385608_tran_6697_op.gml");
    assert_eq!(cityobjs.len(), 549);
    let TopLevelCityObject::Road(road) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Road");
    };

    assert_eq!(
        road.function,
        vec![Code::new("都道府県道".into(), "3".into(),)]
    );
    assert_eq!(
        road.usage,
        vec![
            Code::new("緊急輸送道路（第三次緊急輸送道路）".into(), "3".into()),
            Code::new("避難路／避難道路".into(), "5".into()),
        ]
    );
    assert_eq!(
        road.traffic_area.first().unwrap().function,
        vec![Code::new("歩道".into(), "2020".into())]
    );
    assert_eq!(
        road.auxiliary_traffic_area.first().unwrap().function,
        vec![Code::new("歩道部の段差".into(), "2000".into())]
    );
    assert_eq!(
        road.road_structure_attribute.first().unwrap().width,
        Some(Measure::new(22.0)),
    );
    assert_eq!(
        road.traffic_volume_attribute
            .first()
            .unwrap()
            .weekday12hour_traffic_volume,
        Some(8170),
    );
}

#[test]
fn load_railway_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/rwy/53395527_rwy_6697.gml");
    assert_eq!(cityobjs.len(), 4);
    let TopLevelCityObject::Railway(railway) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Railway");
    };

    assert_eq!(
        railway.id,
        Some("rwy_f087faa5-f548-4188-aa2e-03c7a5f2d3b9".to_string())
    );
    assert_eq!(railway.name, vec!["東北線".to_string()]);
    assert_eq!(railway.traffic_area.len(), 7);
    assert_eq!(
        railway.traffic_area.first().unwrap().function,
        vec![Code::new("軌道中心線".to_string(), "8000".to_string())]
    );
    assert_eq!(railway.auxiliary_traffic_area.len(), 1);
}

#[test]
fn load_track_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/trk/53361601_trk_6697.gml");
    assert_eq!(cityobjs.len(), 125);
    let TopLevelCityObject::Track(track) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Track");
    };

    assert_eq!(track.function, vec![Code::new("徒歩道".into(), "1".into())]);
    assert_eq!(
        track
            .tran_data_quality_attribute
            .as_ref()
            .unwrap()
            .geometry_src_desc,
        vec![Code::new("既成図数値化".into(), "6".into())]
    );
    assert_eq!(
        track.auxiliary_traffic_area.first().unwrap().function,
        vec![Code::new("島".into(), "3000".into())]
    );
    assert_eq!(
        track.track_attribute.first().unwrap().admin_type,
        Some(Code::new("市区町村".into(), "3".into()))
    );
}

#[test]
fn load_square_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/squr/53360690_squr_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::Square(square) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Square");
    };

    assert_eq!(
        square.class,
        Some(Code::new("その他".into(), "1090".into()))
    );
    assert_eq!(
        square.function,
        vec![Code::new("駅前広場".into(), "1".into())]
    );
    assert_eq!(square.traffic_area.len(), 9);
    assert_eq!(square.auxiliary_traffic_area.len(), 3);
    assert_eq!(
        square.traffic_area.first().unwrap().function,
        vec![Code::new("歩道部".into(), "2000".into())]
    );
    assert_eq!(
        square.auxiliary_traffic_area.first().unwrap().function,
        vec![Code::new("島".into(), "3000".into())]
    );
}

#[test]
fn load_waterway_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/wwy/52397630_wwy_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::Waterway(square) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Waterway");
    };

    assert_eq!(
        square.function,
        vec![Code::new("法定航路".into(), "01".into())]
    );
    assert_eq!(
        square.waterway_detail_attribute.as_ref().unwrap().route_id,
        Some("002".into())
    )
}

#[test]
fn load_tunnel_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/tun/53361613_tun_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::Tunnel(tunnel) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a Tunnel");
    };

    assert_eq!(tunnel.class, Some(Code::new("交通".into(), "1000".into())));
    assert_eq!(
        tunnel.function,
        vec![Code::new("道路用トンネル".into(), "1010".into())]
    );
    assert_eq!(tunnel.year_of_construction, Some("1989".into()));
    assert_eq!(
        tunnel.outer_tunnel_installation[0].function,
        vec![Code::new("その他".into(), "90".into())]
    );
    assert_eq!(
        tunnel.outer_tunnel_installation[0].function,
        vec![Code::new("その他".into(), "90".into())]
    );
}

#[test]
fn load_underground_building_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/ubld/51324378_ubld_6697.gml");
    assert_eq!(cityobjs.len(), 3);
    let TopLevelCityObject::UndergroundBuilding(ubld) = &cityobjs.first().unwrap().cityobj else {
        panic!("Not a UndergroundBuilding");
    };
    assert_eq!(ubld.interior_room.len(), 2);
    let room = &ubld.interior_room[1];
    assert_eq!(room.room_installation.len(), 3);
}

#[test]
fn load_urf_example() {
    let cityobjs = load_cityobjs("./tests/data/takeo-shi/udx/urf/493060_urf_6668_op.gml");
    assert_eq!(cityobjs.len(), 140);

    let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/urf/523857_urf_6668_op.gml");
    assert_eq!(cityobjs.len(), 47);

    let cityobjs = load_cityobjs("./tests/data/tokyo23-ku/udx/urf/533957_urf_6668_op.gml");
    assert_eq!(cityobjs.len(), 38);
}

#[test]
fn load_utility_network_example() {
    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/unf/gas_53403039_unf_6697.gml");
        assert_eq!(cityobjs.len(), 7);
        let TopLevelCityObject::OilGasChemicalsPipe(pipe) = &cityobjs[0].cityobj else {
            panic!("expected OilGasChemicalsPipe");
        };
        assert_eq!(pipe.function, vec![Code::new("管路".into(), "5500".into())]);
        let TopLevelCityObject::Appurtenance(appur) = &cityobjs[1].cityobj else {
            panic!("expected Appurtenance");
        };
        assert_eq!(
            appur.function,
            vec![Code::new("ハンドホール".into(), "5620".into())]
        );
        let TopLevelCityObject::Handhole(hole) = &cityobjs[5].cityobj else {
            panic!("expected Handhole");
        };
        assert_eq!(
            hole.function,
            vec![Code::new("ハンドホール".into(), "5620".into())]
        );
    }

    {
        let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/unf/elec_53403039_unf_6697.gml");
        assert_eq!(cityobjs.len(), 2);
        let TopLevelCityObject::Duct(_duct) = &cityobjs[0].cityobj else {
            panic!("unexpected cityobj");
        };
        let TopLevelCityObject::ElectricityCable(_cable) = &cityobjs[1].cityobj else {
            panic!("unexpected cityobj");
        };
    }

    {
        let cityobjs =
            load_cityobjs("./tests/data/plateau-3_0/udx/unf/sewer_53403039_unf_6697.gml");
        assert_eq!(cityobjs.len(), 6);
        let TopLevelCityObject::SewerPipe(_) = &cityobjs[0].cityobj else {
            panic!("expected SewerPipe");
        };
        let TopLevelCityObject::Manhole(_) = &cityobjs[1].cityobj else {
            panic!("expected Manhole");
        };
    }

    {
        let cityobjs =
            load_cityobjs("./tests/data/plateau-3_0/udx/unf/water_53403039_unf_6697.gml");
        assert_eq!(cityobjs.len(), 7);
        let TopLevelCityObject::Appurtenance(_) = &cityobjs[0].cityobj else {
            panic!("expected Appurtenance");
        };
        let TopLevelCityObject::WaterPipe(_) = &cityobjs[1].cityobj else {
            panic!("expected WaterPipe");
        };
    }
}

#[test]
fn load_vegetation_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/veg/52385628_veg_6697_op.gml");
    assert_eq!(cityobjs.len(), 28);
    let TopLevelCityObject::PlantCover(veg) = &cityobjs[0].cityobj else {
        panic!("expected PlantCover");
    };
    assert_eq!(veg.average_height.as_ref().unwrap().value(), 0.5);
    let dq = veg.vegetation_data_quality_attribute.as_ref().unwrap();
    assert_eq!(dq.appearance_src_desc.first().unwrap().code(), "4");

    let TopLevelCityObject::SolitaryVegetationObject(veg) = &cityobjs[9].cityobj else {
        panic!("expected SolitaryVegetationObject");
    };
    assert_eq!(veg.height.as_ref().unwrap().value(), 12.5);
    let dq = veg.vegetation_data_quality_attribute.as_ref().unwrap();
    assert_eq!(dq.appearance_src_desc.first().unwrap().code(), "4");
}

#[test]
fn load_waterbody_example() {
    let cityobjs = load_cityobjs("./tests/data/plateau-3_0/udx/wtr/55370156_wtr_6697.gml");
    assert_eq!(cityobjs.len(), 1);
    let TopLevelCityObject::WaterBody(waterbody) = &cityobjs.first().unwrap().cityobj else {
        panic!("expected WaterBody");
    };

    assert_eq!(
        waterbody.class,
        Some(Code::new(
            "river / stream（河川/小川）".into(),
            "1030".into()
        ))
    );
}

#[test]
fn load_flood_example() {
    {
        let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/fld/52385721_fld_6697_l1_op.gml");
        assert_eq!(cityobjs.len(), 3);
        let TopLevelCityObject::WaterBody(waterbody) = &cityobjs.first().unwrap().cityobj else {
            panic!("expected SedimentDisasterProneArea");
        };
        assert_eq!(waterbody.flooding_risk_attribute.len(), 1);
        let uro::WaterBodyFloodingRiskAttributeProperty::WaterBodyRiverFloodingRiskAttribute(flood) =
            waterbody.flooding_risk_attribute.first().unwrap()
        else {
            panic!("expected WaterBodyRiverFloodingRiskAttribute");
        };
        assert_eq!(flood.admin_type.as_ref().unwrap().code(), "1");
        assert_eq!(flood.scale.as_ref().unwrap().code(), "L1");
    }

    {
        let cityobjs = load_cityobjs("./tests/data/numazu-shi/udx/tnm/523855_tnm_6697_op.gml");
        assert_eq!(cityobjs.len(), 3);
        let TopLevelCityObject::WaterBody(_waterbody) = &cityobjs.first().unwrap().cityobj else {
            panic!("expected SedimentDisasterProneArea");
        };
    }
}