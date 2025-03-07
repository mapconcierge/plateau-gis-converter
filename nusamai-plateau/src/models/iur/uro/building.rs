use nusamai_citygml::{
    citygml_data, citygml_property, CityGmlElement, Code, Date, GYear, Length, Measure,
};

#[citygml_data(name = "uro:BuildingIDAttribute")]
pub struct BuildingIDAttribute {
    #[citygml(path = b"uro:buildingID", required)]
    pub building_id: Option<String>,

    #[citygml(path = b"uro:branchID")]
    pub branch_id: Option<i64>,

    #[citygml(path = b"uro:partID")]
    pub part_id: Option<i64>,

    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"uro:city", required)]
    pub city: Option<Code>,
}

#[citygml_data(name = "uro:BuildingDetailAttribute")]
pub struct BuildingDetailAttribute {
    #[citygml(path = b"uro:serialNumberOfBuildingCertification")]
    pub serial_number_of_building_certification: Option<String>,

    #[citygml(path = b"uro:siteArea")]
    pub site_area: Option<Measure>,

    #[citygml(path = b"uro:totalFloorArea")]
    pub total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:buildingFootprintArea")]
    pub building_footprint_area: Option<Measure>,

    #[citygml(path = b"uro:buildingRoofEdgeArea")]
    pub building_roof_edge_area: Option<Measure>,

    #[citygml(path = b"uro:developmentArea")]
    pub development_area: Option<Measure>,

    #[citygml(path = b"uro:buildingStructureType")]
    pub building_structure_type: Option<Code>,

    #[citygml(path = b"uro:buildingStructureOrgType")]
    pub building_structure_org_type: Option<Code>,

    #[citygml(path = b"uro:fireproofStructureType")]
    pub fireproof_structure_type: Option<Code>,

    #[citygml(path = b"uro:implementingBody")]
    pub implementing_body: Option<String>,

    #[citygml(path = b"uro:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"uro:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"uro:districtsAndZonesType")]
    pub districts_and_zones_type: Vec<Code>,

    #[citygml(path = b"uro:landUseType")]
    pub land_use_type: Option<Code>,

    #[citygml(path = b"uro:reference")]
    pub reference: Option<String>,

    #[citygml(path = b"uro:majorUsage")]
    pub major_usage: Option<Code>,

    #[citygml(path = b"uro:majorUsage2")]
    pub major_usage2: Option<Code>,

    #[citygml(path = b"uro:orgUsage")]
    pub org_usage: Option<Code>,

    #[citygml(path = b"uro:orgUsage2")]
    pub org_usage2: Option<Code>,

    #[citygml(path = b"uro:detailedUsage")]
    pub detailed_usage: Option<Code>,

    #[citygml(path = b"uro:detailedUsage2")]
    pub detailed_usage2: Option<Code>,

    #[citygml(path = b"uro:detailedUsage3")]
    pub detailed_usage3: Option<Code>,

    #[citygml(path = b"uro:groundFloorUsage")]
    pub ground_floor_usage: Option<Code>,

    #[citygml(path = b"uro:secondFloorUsage")]
    pub second_floor_usage: Option<Code>,

    #[citygml(path = b"uro:thirdFloorUsage")]
    pub third_floor_usage: Option<Code>,

    #[citygml(path = b"uro:basementUsage")]
    pub basement_usage: Option<Code>,

    #[citygml(path = b"uro:basementFirstUsage")]
    pub basement_first_usage: Option<Code>,

    #[citygml(path = b"uro:basementSecondUsage")]
    pub basement_second_usage: Option<Code>,

    #[citygml(path = b"uro:vacancy")]
    pub vacancy: Option<Code>,

    #[citygml(path = b"uro:buildingCoverageRate")]
    pub building_coverage_rate: Option<f64>,

    #[citygml(path = b"uro:floorAreaRate")]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"uro:specifiedBuildingCoverageRate")]
    pub specified_building_coverage_rate: Option<f64>,

    #[citygml(path = b"uro:specifiedFloorAreaRate")]
    pub specified_floor_area_rate: Option<f64>,

    #[citygml(path = b"uro:standardFloorAreaRate")]
    pub standard_floor_area_rate: Option<f64>,

    #[citygml(path = b"uro:buildingHeight")]
    pub building_height: Option<Length>,

    #[citygml(path = b"uro:eaveHeight")]
    pub eave_height: Option<Length>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,

    #[citygml(path = b"uro:surveyYear", required)]
    pub survey_year: Option<GYear>,
}

#[citygml_data(name = "uro:BuildingDataQualityAttribute")]
pub struct BuildingDataQualityAttribute {
    #[citygml(path = b"uro:srcScale")]
    pub src_scale: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc")]
    pub geometry_src_desc: Vec<Code>,

    #[citygml(path = b"uro:thematicSrcDesc")]
    pub thematic_src_desc: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDesc")]
    pub appearance_src_desc: Vec<Code>,

    #[citygml(path = b"uro:lod1HeightType", required)]
    pub lod1_height_type: Option<Code>,

    #[citygml(path = b"uro:lodType")]
    pub lod_type: Vec<Code>,
}

#[citygml_property(name = "uro:BuildingDisasterRiskAttributeProperty")]
pub enum BuildingDisasterRiskAttributeProperty {
    #[citygml(path = b"uro:BuildingHighTideRiskAttribute")]
    BuildingHighTideRiskAttribute(BuildingHighTideRiskAttribute),
    #[citygml(path = b"uro:BuildingInlandFloodingRiskAttribute")]
    BuildingInlandFloodingRiskAttribute(BuildingInlandFloodingRiskAttribute),
    #[citygml(path = b"uro:BuildingLandSlideRiskAttribute")]
    BuildingLandSlideRiskAttribute(BuildingLandSlideRiskAttribute),
    #[citygml(path = b"uro:BuildingRiverFloodingRiskAttribute")]
    BuildingRiverFloodingRiskAttribute(BuildingRiverFloodingRiskAttribute),
    #[citygml(path = b"uro:BuildingTsunamiRiskAttribute")]
    BuildingTsunamiRiskAttribute(BuildingTsunamiRiskAttribute),
}

#[citygml_data(name = "uro:BuildingHighTideRiskAttribute")]
pub struct BuildingHighTideRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Measure>,
}

#[citygml_data(name = "uro:BuildingInlandFloodingRiskAttribute")]
pub struct BuildingInlandFloodingRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Measure>,
}

#[citygml_data(name = "uro:BuildingLandSlideRiskAttribute")]
pub struct BuildingLandSlideRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:areaType", required)]
    pub area_type: Option<Code>,
}

#[citygml_data(name = "uro:BuildingTsunamiRiskAttribute")]
pub struct BuildingTsunamiRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Measure>,
}

#[citygml_data(name = "uro:BuildingRiverFloodingRiskAttribute")]
pub struct BuildingRiverFloodingRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Measure>,

    #[citygml(path = b"uro:adminType", required)]
    pub admin_type: Option<Code>,

    #[citygml(path = b"uro:scale", required)]
    pub scale: Option<Code>,

    #[citygml(path = b"uro:duration")]
    pub duration: Option<Measure>,
}

#[citygml_data(name = "uro:RealEstateIDAttribute")]
pub struct RealEstateIDAttribute {
    #[citygml(path = b"uro:realEstateIDOfBuilding", required)]
    pub real_estate_id_of_building: Option<String>,

    #[citygml(path = b"uro:numberOfBuildingUnitOwnership")]
    pub number_of_building_unit_ownership: Option<i64>,

    #[citygml(path = b"uro:realEstateIDOfBuildingUnitOwnership")]
    pub real_estate_id_of_building_unit_ownership: Vec<String>,

    #[citygml(path = b"uro:numberOfRealEstateIDOfLand")]
    pub number_of_real_estate_id_of_land: Option<i64>,

    #[citygml(path = b"uro:realEstateIDOfLand")]
    pub real_estate_id_of_land: Vec<String>,

    #[citygml(path = b"uro:matchingScore", required)]
    pub matching_score: Option<i64>,
}

#[citygml_data(name = "uro:LargeCustomerFacilityAttribute")]
pub struct LargeCustomerFacilityAttribute {
    #[citygml(path = b"uro:class")]
    pub class: Option<Code>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:capacity")]
    pub capacity: Option<i64>,

    #[citygml(path = b"uro:owner")]
    pub owner: Option<String>,

    #[citygml(path = b"uro:totalFloorArea")]
    pub total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:totalStoreFloorArea")]
    pub total_store_floor_area: Option<Measure>,

    #[citygml(path = b"uro:inauguralDate")]
    pub inaugural_date: Option<Date>,

    #[citygml(path = b"uro:yearOpened")]
    pub year_opened: Option<GYear>,

    #[citygml(path = b"uro:yearClosed")]
    pub year_closed: Option<GYear>,

    #[citygml(path = b"uro:keyTenants")]
    pub key_tenants: Option<String>,

    #[citygml(path = b"uro:availability")]
    pub availability: Option<bool>,

    #[citygml(path = b"uro:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"uro:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"uro:districtsAndZonesType")]
    pub districts_and_zones_type: Vec<Code>,

    #[citygml(path = b"uro:landUseType")]
    pub land_use_type: Option<Code>,

    #[citygml(path = b"uro:reference")]
    pub reference: Option<String>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,

    #[citygml(path = b"uro:surveyYear", required)]
    pub survey_year: Option<GYear>,
}

#[citygml_data(name = "uro:RoomDataQualityAttribute")]
pub struct RoomDataQualityAttribute {
    #[citygml(path = b"uro:srcScale")]
    pub src_scale: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc")]
    pub geometry_src_desc: Vec<Code>,

    #[citygml(path = b"uro:thematicSrcDesc")]
    pub thematic_src_desc: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDesc")]
    pub appearance_src_desc: Vec<Code>,

    #[citygml(path = b"uro:lodType")]
    pub lod_type: Option<Code>,
}
