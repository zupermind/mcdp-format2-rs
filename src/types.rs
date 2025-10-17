#![allow(unused_imports)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused_qualifications)]
pub mod concrete {
    use serde::{Serialize, Deserialize};
    use serde_json::Value as AnyValue;
    use std::sync::Arc;
    /// Specifies the origin of an object from a repo and a library.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Address {
        /// The library name
        pub library: String,
        /// The Git repository URL
        pub repo: Option<String>,
        /// The type of object
        pub spec: String,
        /// The name of the object
        pub thing: String,
        /// Type marker
        pub r#type: String,
    }
    /// Checks for the maps, as used in test cases.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum Check {
        /// Check for a L1Map.
        L1Check(L1Check),
        /// Check for a LMap.
        LCheck(LCheck),
        /// Check for a monotone map.
        MapCheck(MapCheck),
        /// Check for a SL1Map.
        SL1Check(SL1Check),
        /// Check for a SL1Map.
        SLCheck(SLCheck),
        /// Check for a SU1Map.
        SU1Check(SU1Check),
        /// Check for a SUMap.
        SUCheck(SUCheck),
        /// Check for a U1Map.
        U1Check(U1Check),
        /// Check for a UMap.
        UCheck(UCheck),
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum ComputeJob {
        ComputeJob_F_to_R(ComputeJob_F_to_R),
        ComputeJob_R_to_F(ComputeJob_R_to_F),
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ComputeJob_F_to_R {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub address: Option<Box<Address>>,
        /// The names of the axes of the computation job.
        pub axes: Option<std::collections::HashMap<String, i64>>,
        pub points: Option<Vec<Box<ComputePoint>>>,
        pub f_b_r: Box<SUMap>,
        pub f_i_r: Box<SUMap>,
        pub f_r: Box<SU1Map>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ComputeJob_R_to_F {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub address: Option<Box<Address>>,
        /// The names of the axes of the computation job.
        pub axes: Option<std::collections::HashMap<String, i64>>,
        pub points: Option<Vec<Box<ComputePoint>>>,
        pub r_b_f: Box<SLMap>,
        pub r_f: Box<SL1Map>,
        pub r_i_f: Box<SLMap>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ComputePoint {
        /// Key of the point, to be referenced later.
        pub key: Option<String>,
        /// Value of the point
        pub value: Option<AnyValue>,
    }
    /// Represents a connection between two nodes in the NDP graph
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Connection {
        /// The source of the connection.
        pub source: Box<ConnectionSource>,
        /// The target of the connection.
        pub target: Box<ConnectionTarget>,
        /// Type marker.
        pub r#type: String,
    }
    /// The source of a connection.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum ConnectionSource {
        /// The source of a connection is a functionality of the composite graph.
        ModelFunctionality(ModelFunctionality),
        /// The source of a connection is a requirement of another node.
        NodeRequirement(NodeRequirement),
    }
    /// The target of a connection.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum ConnectionTarget {
        /// The target is the requirement of the ambient model.
        ModelRequirement(ModelRequirement),
        /// The target is the functionality of another subproblem.
        NodeFunctionality(NodeFunctionality),
    }
    /// Design problem with implementations (DPI)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum DP {
        /// Compares resources to a function and a set of constants (conjunction).
        DP_All_Constants_And_F_Leq_R(DP_All_Constants_And_F_Leq_R),
        /// Compare a resource to a set of constants
        DP_All_Constants_Leq_R(DP_All_Constants_Leq_R),
        /// Compares a vector of functions to a resource (conjunction).
        DP_All_Fi_Leq_R(DP_All_Fi_Leq_R),
        /// Compares functionality and resources in an ambient poset.
        DP_AmbientConversion(DP_AmbientConversion),
        /// Compares resources to a function and a set of constants (disjunction).
        DP_Any_Constants_Or_F_Leq_R(DP_Any_Constants_Or_F_Leq_R),
        /// Compares a vector of functions to a resource (disjunction).
        DP_Any_Fi_Leq_R(DP_Any_Fi_Leq_R),
        /// Multi-resolution DP
        DP_C_ExplicitApprox(DP_C_ExplicitApprox),
        /// Intersection of design problems
        DP_C_Intersection(DP_C_Intersection),
        /// Monoidal product of design problems.
        DP_C_Parallel(DP_C_Parallel),
        /// Series composition of DPs.
        DP_C_Series(DP_C_Series),
        /// Trace of a design problem.
        DP_C_Trace(DP_C_Trace),
        /// Union of design problems (DPs).
        DP_C_Union(DP_C_Union),
        /// A DP defined explicitly by a set of options.
        DP_Catalog(DP_Catalog),
        /// An "opaque" DP defined explicitly by its interface.
        DP_Compiled(DP_Compiled),
        /// Compare a functionality to a set of constants
        DP_F_Leq_All_Constants(DP_F_Leq_All_Constants),
        /// Compares a functionality to a resource and a set of constants (conjunction).
        DP_F_Leq_All_R_And_Constants(DP_F_Leq_All_R_And_Constants),
        /// Compares a vector of resources to a function (conjunction).
        DP_F_Leq_All_Ri(DP_F_Leq_All_Ri),
        /// Compares a functionality to a resource and a set of constants (disjunction).
        DP_F_Leq_Any_R_And_Constants(DP_F_Leq_Any_R_And_Constants),
        /// Compares a vector of resources to a function (disjunction).
        DP_F_Leq_Any_Ri(DP_F_Leq_Any_Ri),
        /// The DP that is always false.
        DP_False(DP_False),
        /// Identity with limit to the functionality.
        DP_FuncNotMoreThan(DP_FuncNotMoreThan),
        /// A DP with exactly one implementation.
        DP_GenericConstant(DP_GenericConstant),
        /// The identity design problem.
        DP_Identity(DP_Identity),
        /// Enforces isomorphism between functionalities and requirements.
        DP_Iso(DP_Iso),
        /// A DP generated from a monotone map from requirements to functionalities.
        DP_LiftL(DP_LiftL),
        /// A DP generated from a monotone map from functionality to requirements.
        DP_LiftU(DP_LiftU),
        /// Identity with limit to the resource.
        DP_ResNotLessThan(DP_ResNotLessThan),
        /// The DP that is always true.
        DP_True(DP_True),
        /// Placeholder for an unknown design problem.
        DP_Unknown(DP_Unknown),
    }
    /// Compares resources to a function and a set of constants (conjunction).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_All_Constants_And_F_Leq_R {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of constants c₁, ..., c_n.
        pub constants: Vec<AnyValue>,
    }
    /// Compare a resource to a set of constants
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_All_Constants_Leq_R {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of constants c₁, ..., c_n.
        pub constants: Vec<AnyValue>,
    }
    /// Compares a vector of functions to a resource (conjunction).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_All_Fi_Leq_R {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// Compares functionality and resources in an ambient poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_AmbientConversion {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub common: Box<Poset>,
    }
    /// Compares resources to a function and a set of constants (disjunction).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_Any_Constants_Or_F_Leq_R {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of constants c₁, ..., c_n.
        pub constants: Vec<AnyValue>,
    }
    /// Compares a vector of functions to a resource (disjunction).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_Any_Fi_Leq_R {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// Multi-resolution DP
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_C_ExplicitApprox {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// List of optimistic DPs.
        pub optimistic: Vec<Box<DP>>,
        /// Labels for the optimistic DPs.
        pub optimistic_labels: Option<Vec<String>>,
        /// List of pessimistic DPs.
        pub pessimistic: Vec<Box<DP>>,
        /// Labels for the pessimistic DPs.
        pub pessimistic_labels: Option<Vec<String>>,
    }
    /// Intersection of design problems
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of design problems (DPs) to be composed.
        pub dps: Vec<Box<DP>>,
        /// A list of labels.
        pub labels: Option<Vec<String>>,
    }
    /// Monoidal product of design problems.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of design problems (DPs) to be composed.
        pub dps: Vec<Box<DP>>,
        /// A list of labels.
        pub labels: Option<Vec<String>>,
    }
    /// Series composition of DPs.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of design problems (DPs) to be composed.
        pub dps: Vec<Box<DP>>,
        /// A list of labels.
        pub labels: Option<Vec<String>>,
    }
    /// Trace of a design problem.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The design problem that is being traced.
        pub dp: Box<DP>,
    }
    /// Union of design problems (DPs).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of design problems (DPs) to be composed.
        pub dps: Vec<Box<DP>>,
        /// A list of labels.
        pub labels: Option<Vec<String>>,
    }
    /// A DP defined explicitly by a set of options.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_Catalog {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /** A list of options that define the design problem.
Each option is a tuple of functionality, requirement, blueprint, and implementation.*/
        pub options: Vec<Box<DP_Catalog_Options>>,
    }
    /// One option for the catalog
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_Catalog_Options {
        /// Blueprint
        pub b: AnyValue,
        /// Functionality
        pub f: AnyValue,
        /// Implementation
        pub i: AnyValue,
        /// Requirement
        pub r: AnyValue,
    }
    /// An "opaque" DP defined explicitly by its interface.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_Compiled {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The function that returns the maximum functionality given the budget of resources as well as the blueprint.
        pub f_b_r: Box<SUMap>,
        /// The function that returns the maximum functionality given the budget of resources as well as the implementation.
        pub f_i_r: Box<SUMap>,
        /// The function that returns minimal resources needed to satisfy the requirements.
        pub f_r: Box<SU1Map>,
        /// The function that maps implementations to their availability.
        pub i_availability: Box<MonotoneMap>,
        /// The function that maps implementations to blueprints.
        pub i_b: Box<MonotoneMap>,
        /// The function that maps implementations to their internal feasibility.
        pub i_codfeas: Box<MonotoneMap>,
        /// The "provides" map from implementations to functionalities.
        pub prov: Box<MonotoneMap>,
        /// The function that returns the maximum functionality given the budget of resources as well as the blueprint.
        pub r_b_f: Box<SLMap>,
        /// The function that returns the maximum functionality given the budget of requirements.
        pub r_f: Box<SL1Map>,
        /// The function that returns the maximum functionality given the budget of resources as well as the implementation.
        pub r_i_f: Box<SLMap>,
        /// The "requires" map from implementations to requirements.
        pub req: Box<MonotoneMap>,
    }
    /// Compare a functionality to a set of constants
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_F_Leq_All_Constants {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of constants c₁, ..., c_n.
        pub constants: Vec<AnyValue>,
    }
    /// Compares a functionality to a resource and a set of constants (conjunction).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_F_Leq_All_R_And_Constants {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of constants c₁, ..., c_n.
        pub constants: Vec<AnyValue>,
    }
    /// Compares a vector of resources to a function (conjunction).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_F_Leq_All_Ri {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// Compares a functionality to a resource and a set of constants (disjunction).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_F_Leq_Any_R_And_Constants {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of constants c₁, ..., c_n.
        pub constants: Vec<AnyValue>,
    }
    /// Compares a vector of resources to a function (disjunction).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_F_Leq_Any_Ri {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// The DP that is always false.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_False {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// Identity with limit to the functionality.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_FuncNotMoreThan {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub limit: AnyValue,
    }
    /// A DP with exactly one implementation.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_GenericConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub b_value: AnyValue,
        pub lower_set: Box<LowerSet>,
        pub upper_set: Box<UpperSet>,
    }
    /// The identity design problem.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// Enforces isomorphism between functionalities and requirements.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_Iso {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub bwd: Box<MonotoneMap>,
        pub fwd: Box<MonotoneMap>,
    }
    /// A DP generated from a monotone map from requirements to functionalities.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_LiftL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub m: Box<MonotoneMap>,
    }
    /// A DP generated from a monotone map from functionality to requirements.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_LiftU {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A monotone map from the poset of functionalities to the poset of requirements.
        pub m: Box<MonotoneMap>,
    }
    /// Identity with limit to the resource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_ResNotLessThan {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub limit: AnyValue,
    }
    /// The DP that is always true.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_True {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The implementation value.
        pub value: Box<Value>,
    }
    /// Placeholder for an unknown design problem.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DP_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Poset of blueprints. If not present, it is the smash unit.
        pub B: Option<Box<Poset>>,
        /// Poset of functionalities
        pub F: Box<Poset>,
        /// Poset of implementations. If not present, it is the smash unit.
        pub I: Option<Box<Poset>>,
        /// Poset of requirements
        pub R: Box<Poset>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// Check for a L1Map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1Check {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Data to check the L1Map.
        pub data: Vec<Box<L1Check_Data>>,
        /// The map to check
        pub m: Box<L1Map>,
    }
    /// An input-output pair for the L1Map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1Check_Data {
        /// Time taken for the check in seconds (optional).
        pub elapsed: Option<f64>,
        pub x: AnyValue,
        /// Expected result (a lower set).
        pub y: Box<LowerSet>,
    }
    /// Map to lower sets of functionalities.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum L1Map {
        /// Co-domain sum combination
        L1_C_CodSum(L1_C_CodSum),
        /// Co-domain (smash) sum combination
        L1_C_CodSumSmash(L1_C_CodSumSmash),
        /// Domain union
        L1_C_DomUnion(L1_C_DomUnion),
        /// Intersection
        L1_C_Intersection(L1_C_Intersection),
        /// Monoidal product
        L1_C_Parallel(L1_C_Parallel),
        /// From product to intersection
        L1_C_ProdIntersection(L1_C_ProdIntersection),
        /// Product
        L1_C_Product(L1_C_Product),
        /// Refines the domain of a monotone map.
        L1_C_RefineDomain(L1_C_RefineDomain),
        /// Series composition
        L1_C_Series(L1_C_Series),
        /// Trace
        L1_C_Trace(L1_C_Trace),
        /// Union
        L1_C_Union(L1_C_Union),
        /// Decorates a map with units.
        L1_C_WrapUnits(L1_C_WrapUnits),
        /// Map induced by a catalog of options.
        L1_Catalog(L1_Catalog),
        /// Constant map
        L1_Constant(L1_Constant),
        /// Returns the entire poset
        L1_Entire(L1_Entire),
        /// Map defined pointwise
        L1_Explicit(L1_Explicit),
        /// Filters based on a monotone map.
        L1_FromFilter(L1_FromFilter),
        /// Lift of the identity map
        L1_Identity(L1_Identity),
        /// Intersection of principal lower sets.
        L1_IntersectionOfPrinLowerSets(L1_IntersectionOfPrinLowerSets),
        /// Finite-resolution optimistic approximation of the inverse of a multiplication map.
        L1_InvMul_Opt(L1_InvMul_Opt),
        /// Finite-resolution pessimistic approximation of the inverse of an addition map.
        L1_InvMul_Pes(L1_InvMul_Pes),
        /// Finite-resolution optimistic approximation of the inverse of a multiplication map.
        L1_InvSum_Opt(L1_InvSum_Opt),
        /// Finite-resolution pessimistic approximation of the inverse of an addition map.
        L1_InvSum_Pes(L1_InvSum_Pes),
        /// Lower inverse of a monotone map
        L1_L_Linv(L1_L_Linv),
        /// Lifts a monotone map
        L1_Lift(L1_Lift),
        /// Represent a principal lower set
        L1_RepresentPrincipalLowerSet(L1_RepresentPrincipalLowerSet),
        /// Lower inverse for the meet map
        L1_TopAlternating(L1_TopAlternating),
        /// Union of principal lower sets.
        L1_UnionOfPrinLowerSets(L1_UnionOfPrinLowerSets),
        /// Placeholder for an unknown map.
        L1_Unknown(L1_Unknown),
    }
    /// Co-domain sum combination
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_CodSum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// Co-domain (smash) sum combination
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_CodSumSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// Domain union
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_DomUnion {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// Intersection
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// Monoidal product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// From product to intersection
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_ProdIntersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// Product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_Product {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// Refines the domain of a monotone map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<L1Map>,
    }
    /// Series composition
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// Trace
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<L1Map>,
    }
    /// Union
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<L1Map>>,
    }
    /// Decorates a map with units.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// The units for the codomain
        pub kcod_units: Box<Unit>,
        /// The units for the domain
        pub kdom_units: Box<Unit>,
        pub m: Box<L1Map>,
    }
    /// Map induced by a catalog of options.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Catalog {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub options: Vec<Box<L1_Catalog_Options>>,
    }
    /// An option in the catalog
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Catalog_Options {
        pub f: AnyValue,
        pub r: AnyValue,
    }
    /// Constant map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub value: Box<LowerSet>,
    }
    /// Returns the entire poset
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Entire {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Map defined pointwise
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Explicit {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Pairs of input-output
        pub options: Vec<Box<L1_Explicit_Option>>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Explicit_Option {
        /// A point in the domain of the map.
        pub x: AnyValue,
        /// The lower set corresponding to the point `x` in the domain.
        pub y: Box<LowerSet>,
    }
    /// Filters based on a monotone map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_FromFilter {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<MonotoneMap>,
    }
    /// Lift of the identity map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Intersection of principal lower sets.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_IntersectionOfPrinLowerSets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Finite-resolution optimistic approximation of the inverse of a multiplication map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_InvMul_Opt {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Resolution (number of points in the produced antichain)
        pub n: i64,
        /// The poset in which the operation is performed.
        pub opspace: Box<Poset>,
    }
    /// Finite-resolution pessimistic approximation of the inverse of an addition map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_InvMul_Pes {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Resolution (number of points in the produced antichain)
        pub n: i64,
        /// The poset in which the operation is performed.
        pub opspace: Box<Poset>,
    }
    /// Finite-resolution optimistic approximation of the inverse of a multiplication map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_InvSum_Opt {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Resolution (number of points in the produced antichain)
        pub n: i64,
        /// The poset in which the operation is performed.
        pub opspace: Box<Poset>,
    }
    /// Finite-resolution pessimistic approximation of the inverse of an addition map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_InvSum_Pes {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Resolution (number of points in the produced antichain)
        pub n: i64,
        /// The poset in which the operation is performed.
        pub opspace: Box<Poset>,
    }
    /// Lower inverse of a monotone map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_L_Linv {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<MonotoneMap>,
    }
    /// Lifts a monotone map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Lift {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<MonotoneMap>,
    }
    /// Represent a principal lower set
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_RepresentPrincipalLowerSet {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Lower inverse for the meet map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_TopAlternating {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub upper_bounds: Vec<Vec<AnyValue>>,
    }
    /// Union of principal lower sets.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_UnionOfPrinLowerSets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Placeholder for an unknown map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Check for a LMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LCheck {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Test pairs
        pub data: Vec<Box<LCheck_Data>>,
        /// The map to check
        pub m: Box<LMap>,
    }
    /// An input-output pair for the map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LCheck_Data {
        /// Time taken for the check in seconds.
        pub elapsed: Option<f64>,
        pub x: AnyValue,
        pub y: Box<LowerSet>,
    }
    /// Map to lower sets of functionalities and implementations.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum LMap {
        /// Transforms the implementation of another map.
        L_C_ITransform(L_C_ITransform),
        /// Intersection of maps
        L_C_Intersection(L_C_Intersection),
        /// Monoidal product
        L_C_Parallel(L_C_Parallel),
        /// Refines the domain of a monotone map
        L_C_RefineDomain(L_C_RefineDomain),
        /// Series composition
        L_C_Series(L_C_Series),
        /// Trace
        L_C_Trace(L_C_Trace),
        /// Trace (second version with extra imp)
        L_C_TraceL(L_C_TraceL),
        /// Union of maps
        L_C_Union(L_C_Union),
        /// Decorates a map with units.
        L_C_WrapUnits(L_C_WrapUnits),
        /// LMap for a catalog
        L_Catalog(L_Catalog),
        /// Constant map
        L_Constant(L_Constant),
        /// Identity morphism
        L_Identity(L_Identity),
        /// Lifts a L1Map morphisms with a constant value for the implementation.
        L_L_Lift1_Constant(L_L_Lift1_Constant),
        /// Lifts a L1Map morphism with a function to compute the implementation.
        L_L_Lift1_Transform(L_L_Lift1_Transform),
        /// Placeholder for an unknown map
        L_Unknown(L_Unknown),
    }
    /// Transforms the implementation of another map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_ITransform {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<LMap>,
        pub transform: Box<MonotoneMap>,
    }
    /// Intersection of maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// Labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<LMap>>,
    }
    /// Monoidal product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// Labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<LMap>>,
    }
    /// Refines the domain of a monotone map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<LMap>,
    }
    /// Series composition
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// Labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<LMap>>,
    }
    /// Trace
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<LMap>,
        pub m_proj: Box<L1Map>,
    }
    /// Trace (second version with extra imp)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_TraceL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<LMap>,
        pub m_proj: Box<L1Map>,
    }
    /// Union of maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// Labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<LMap>>,
    }
    /// Decorates a map with units.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub kcod_units: Box<Unit>,
        pub kdom_units: Box<Unit>,
        pub kimp_units: Box<Unit>,
        pub m: Box<LMap>,
    }
    /// LMap for a catalog
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_Catalog {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub options: Vec<Box<L_Catalog_Options>>,
    }
    /// Options for L_Catalog
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_Catalog_Options {
        pub f: AnyValue,
        pub i: AnyValue,
        pub r: AnyValue,
    }
    /// Constant map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// The lower set that is the value of the constant map.
        pub value: Box<LowerSet>,
    }
    /// Identity morphism
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
    }
    /// Lifts a L1Map morphisms with a constant value for the implementation.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_L_Lift1_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<L1Map>,
        pub value: AnyValue,
    }
    /// Lifts a L1Map morphism with a function to compute the implementation.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_L_Lift1_Transform {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<L1Map>,
        pub transform: Box<MonotoneMap>,
    }
    /// Placeholder for an unknown map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
    }
    /// Represents a lower set in a poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum LowerSet {
        /// A lower set defined as the down closure of a finite set of points.
        LowerSet_LowerClosure(LowerSet_LowerClosure),
        /// Unused
        LowerSet_Unused(LowerSet_Unused),
    }
    /// A lower set defined as the down closure of a finite set of points.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LowerSet_LowerClosure {
        /// Kind marker.
        pub kind: String,
        pub points: Vec<AnyValue>,
    }
    /// Unused
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LowerSet_Unused {
        /// Kind marker.
        pub kind: String,
    }
    /// Addition in the L topology.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_AddL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspaces: Vec<Box<Poset>>,
    }
    /// Add a constant in the L topology.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_AddLConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Addition in the U topology.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_AddU {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspaces: Vec<Box<Poset>>,
    }
    /// Addition of constant in the U topology.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_AddUConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Maps top to top, and everything else to bottom.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_BottomIfNotTop {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Coproduct of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_Coproduct {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Smash coproduct of two monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_CoproductSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// A monotone map from a product of domains to a smash product of codomains.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_DomProdCodSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// A monotone map from the smash product of domains to the product of codomains.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_DomSmashCodProd {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Domain union of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_DomUnion {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Tests $\text{constant} \leq x$
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_Leq_X {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Poset in which the comparison is performed.
        pub opspace: Box<Poset>,
        /// Comparison value.
        pub value: Box<Value>,
    }
    /// Lift of a monotone map to subsets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_LiftToSubsets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// The monotone map that is lifted.
        pub m: Box<MonotoneMap>,
    }
    /// Tests $\text{constant} < x$
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_Lt_X {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Poset in which the comparison is performed.
        pub opspace: Box<Poset>,
        /// Comparison value.
        pub value: Box<Value>,
    }
    /// Opposite of a map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_Op {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub m: Box<MonotoneMap>,
    }
    /// Monoidal product of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Monoidal (smash) product of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_ParallelSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Product of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_Product {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Smash product of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_ProductSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// A refinement of the domain of a monotone map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub m: Box<MonotoneMap>,
    }
    /// Series composition of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Sum of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_Sum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Smash sum of monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_SumSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// A list of labels for the monotone maps.
        pub labels: Option<Vec<String>>,
        pub maps: Vec<Box<MonotoneMap>>,
    }
    /// Wraps a monotone map with units descriptions for domain and codomain.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Units for the codomain of the monotone map.
        pub cod_units: Box<Unit>,
        /// Units for the domain of the monotone map.
        pub dom_units: Box<Unit>,
        pub m: Box<MonotoneMap>,
    }
    /// Ceiling function relative
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Ceil0 {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
    }
    /// Coerces from one poset to another
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Coerce {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// A constant function
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Test for containment in a lower set
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_ContainedInLowerSet {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Must be a boolean poset.
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub lower_set: Box<LowerSet>,
        pub opspace: Box<Poset>,
    }
    /// Test for containment in an upper set
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_ContainedInUpperSet {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Must be a boolean poset.
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub upper_set: Box<UpperSet>,
    }
    /// Division by a constant (L topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_DivideLConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Division by a constant (U topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_DivideUConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// The unique map from the empty set to another
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Empty {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// A map defined pointwise.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Explicit {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub options: Vec<Box<M_Explicit_Option>>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Explicit_Option {
        pub x: AnyValue,
        pub y: AnyValue,
    }
    /// Floor function relative
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Floor0 {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
    }
    /// Identity map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Id {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// A monotone map that outputs a constant value if the input is above a threshold.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_IdentityBelowThreshold {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Threshold value.
        pub threshold: Box<Value>,
        /** Value returned by the map if the input is above the threshold.
This value must be greater than or equal to the threshold.*/
        pub value: Box<Value>,
    }
    /// Injection into a poset sum
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Injection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Which space to inject into
        pub index: i64,
    }
    /// Join operation
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Join {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// The posets in which each join is defined.
        pub opspaces: Vec<Box<Poset>>,
    }
    /// Join with a constant value
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_JoinConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Tests $x_1 \leq_P x_2$
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Leq {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Poset in which the comparison is performed.
        pub opspace: Box<Poset>,
    }
    /// Lifts a value to a tuple with one element.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Lift {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Lifts a monotone map to lower sets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_LiftToLowerSets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// The monotone map that is lifted.
        pub m: Box<MonotoneMap>,
    }
    /// Lifts a monotone map to upper sets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_LiftToUpperSets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// The monotone map that is lifted.
        pub m: Box<MonotoneMap>,
    }
    /// Meet operation
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Meet {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// The posets in which each meet is defined.
        pub opspaces: Vec<Box<Poset>>,
    }
    /// Meet with a constant
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_MeetConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Multiplication (L topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_MultiplyL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspaces: Vec<Box<Poset>>,
    }
    /// Multiplication by a constant (L topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_MultiplyLConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Multiplication (U topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_MultiplyU {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspaces: Vec<Box<Poset>>,
    }
    /// Multiplication by a constant (U topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_MultiplyUConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Lift to the power of a fraction (L topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_PowerFracL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub den: String,
        pub num: String,
        pub opspace: Box<Poset>,
    }
    /// Lift to the power of a fraction (U topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_PowerFracU {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub den: String,
        pub num: String,
        pub opspace: Box<Poset>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_ReprLowerSet {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Maps a point to the largest upper set containing its closure
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_ReprUpperSet {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Largest principal lower set in the poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_RepresentPrincipalLowerSet_TotalOrderBounded {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Largest principal upper set in the poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_RepresentPrincipalUpperSet_TotalOrderBounded {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Round down
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_RoundDown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub offset: AnyValue,
        pub opspace: Box<Poset>,
        pub step: String,
    }
    /// Round up
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_RoundUp {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub offset: AnyValue,
        pub opspace: Box<Poset>,
        pub step: String,
    }
    /// Scaling in the L topology by a fraction.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_ScaleL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub den: String,
        pub num: String,
        pub opspace: Box<Poset>,
    }
    /// Scaling in the U topology by a fraction.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_ScaleU {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub den: String,
        pub num: String,
        pub opspace: Box<Poset>,
    }
    /// Injection into a smash sum
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_SmashInjection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Which space to inject into
        pub index: i64,
    }
    /// Subtraction of a constant (L topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_SubLConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Subtraction by a constant (U topology)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_SubUConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub opspace: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Projection of an element in a poset product.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_TakeIndex {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Describes the projection
        pub projection: Box<Projection>,
    }
    /// Projection of a range of elements in a smash poset product.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_TakeRange {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Describes the range of indices to take.
        pub range: Box<Range>,
    }
    /// Threshold map (r-to-f for DP_FuncNotMoreThan)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Threshold1 {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Threshold map (f-to-r for DP_ResNotLessThan)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Threshold2 {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Maps bottom to bottom, and everything else to top.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_TopIfNotBottom {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Undefined map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Undefined {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Placeholder for an unknown map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Unlifts a one-element tuple to its single element.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_Unlift {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
    }
    /// Tests $x \leq \text{constant}$
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_X_Leq_C {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Poset in which the comparison is performed.
        pub opspace: Box<Poset>,
        /// Comparison value.
        pub value: Box<Value>,
    }
    /// Tests $x < \text{constant}$
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct M_X_Lt_C {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Codomain of the monotone map
        pub cod: Box<Poset>,
        /// Domain of the monotone map
        pub dom: Box<Poset>,
        /// Poset in which the comparison is performed.
        pub opspace: Box<Poset>,
        /// Comparison value.
        pub value: Box<Value>,
    }
    /// Check for a monotone map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MapCheck {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Test pairs
        pub data: Vec<Box<MapCheck_Data>>,
        /// The map to check.
        pub m: Box<MonotoneMap>,
    }
    /// An input-output pair for the map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MapCheck_Data {
        /// Time taken to compute the result.
        pub elapsed: Option<f64>,
        pub x: AnyValue,
        pub y: AnyValue,
    }
    /// The source of a connection is a functionality of the composite graph.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ModelFunctionality {
        pub functionality: String,
    }
    /// The target is the requirement of the ambient model.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ModelRequirement {
        pub requirement: String,
    }
    /// Monotone maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum MonotoneMap {
        /// Addition in the L topology.
        M_AddL(M_AddL),
        /// Add a constant in the L topology.
        M_AddLConstant(M_AddLConstant),
        /// Addition in the U topology.
        M_AddU(M_AddU),
        /// Addition of constant in the U topology.
        M_AddUConstant(M_AddUConstant),
        /// Maps top to top, and everything else to bottom.
        M_BottomIfNotTop(M_BottomIfNotTop),
        /// Coproduct of monotone maps
        M_C_Coproduct(M_C_Coproduct),
        /// Smash coproduct of two monotone maps
        M_C_CoproductSmash(M_C_CoproductSmash),
        /// A monotone map from a product of domains to a smash product of codomains.
        M_C_DomProdCodSmash(M_C_DomProdCodSmash),
        /// A monotone map from the smash product of domains to the product of codomains.
        M_C_DomSmashCodProd(M_C_DomSmashCodProd),
        /// Domain union of monotone maps
        M_C_DomUnion(M_C_DomUnion),
        /// Tests constant LEQ value.
        M_C_Leq_X(M_C_Leq_X),
        /// Lift of a monotone map to subsets
        M_C_LiftToSubsets(M_C_LiftToSubsets),
        /// Tests constant LT value.
        M_C_Lt_X(M_C_Lt_X),
        /// Opposite of a map
        M_C_Op(M_C_Op),
        /// Monoidal product of monotone maps
        M_C_Parallel(M_C_Parallel),
        /// Monoidal (smash) product of monotone maps
        M_C_ParallelSmash(M_C_ParallelSmash),
        /// Product of monotone maps
        M_C_Product(M_C_Product),
        /// Smash product of monotone maps
        M_C_ProductSmash(M_C_ProductSmash),
        /// A refinement of the domain of a monotone map
        M_C_RefineDomain(M_C_RefineDomain),
        /// Series composition of monotone maps
        M_C_Series(M_C_Series),
        /// Sum of monotone maps
        M_C_Sum(M_C_Sum),
        /// Smash sum of monotone maps
        M_C_SumSmash(M_C_SumSmash),
        /// Wraps a monotone map with units descriptions for domain and codomain.
        M_C_WrapUnits(M_C_WrapUnits),
        /// Ceiling function relative
        M_Ceil0(M_Ceil0),
        /// Coerces from one poset to another
        M_Coerce(M_Coerce),
        /// A constant function
        M_Constant(M_Constant),
        /// Test for containment in a lower set
        M_ContainedInLowerSet(M_ContainedInLowerSet),
        /// Test for containment in an upper set
        M_ContainedInUpperSet(M_ContainedInUpperSet),
        /// Division by a constant (L topology)
        M_DivideLConstant(M_DivideLConstant),
        /// Division by a constant (U topology)
        M_DivideUConstant(M_DivideUConstant),
        /// The unique map from the empty set to another
        M_Empty(M_Empty),
        /// A map defined pointwise.
        M_Explicit(M_Explicit),
        /// Floor function relative
        M_Floor0(M_Floor0),
        /// Identity map
        M_Id(M_Id),
        /// A monotone map that outputs a constant value if the input is above a threshold.
        M_IdentityBelowThreshold(M_IdentityBelowThreshold),
        /// Injection into a poset sum
        M_Injection(M_Injection),
        /// Join operation
        M_Join(M_Join),
        /// Join with a constant value
        M_JoinConstant(M_JoinConstant),
        /// Tests v1 LEQ v2.
        M_Leq(M_Leq),
        /// Lifts a value to a tuple with one element.
        M_Lift(M_Lift),
        /// Lifts a monotone map to lower sets
        M_LiftToLowerSets(M_LiftToLowerSets),
        /// Lifts a monotone map to upper sets
        M_LiftToUpperSets(M_LiftToUpperSets),
        /// Meet operation
        M_Meet(M_Meet),
        /// Meet with a constant
        M_MeetConstant(M_MeetConstant),
        /// Multiplication (L topology)
        M_MultiplyL(M_MultiplyL),
        /// Multiplication by a constant (L topology)
        M_MultiplyLConstant(M_MultiplyLConstant),
        /// Multiplication (U topology)
        M_MultiplyU(M_MultiplyU),
        /// Multiplication by a constant (U topology)
        M_MultiplyUConstant(M_MultiplyUConstant),
        /// Lift to the power of a fraction (L topology)
        M_PowerFracL(M_PowerFracL),
        /// Lift to the power of a fraction (U topology)
        M_PowerFracU(M_PowerFracU),
        M_ReprLowerSet(M_ReprLowerSet),
        /// Maps a point to the largest upper set containing its closure
        M_ReprUpperSet(M_ReprUpperSet),
        /// Largest principal lower set in the poset.
        M_RepresentPrincipalLowerSet_TotalOrderBounded(
            M_RepresentPrincipalLowerSet_TotalOrderBounded,
        ),
        /// Largest principal upper set in the poset.
        M_RepresentPrincipalUpperSet_TotalOrderBounded(
            M_RepresentPrincipalUpperSet_TotalOrderBounded,
        ),
        /// Round down
        M_RoundDown(M_RoundDown),
        /// Round up
        M_RoundUp(M_RoundUp),
        /// Scaling in the L topology by a fraction.
        M_ScaleL(M_ScaleL),
        /// Scaling in the U topology by a fraction.
        M_ScaleU(M_ScaleU),
        /// Injection into a smash sum
        M_SmashInjection(M_SmashInjection),
        /// Subtraction of a constant (L topology)
        M_SubLConstant(M_SubLConstant),
        /// Subtraction by a constant (U topology)
        M_SubUConstant(M_SubUConstant),
        /// Projection of an element in a poset product.
        M_TakeIndex(M_TakeIndex),
        /// Projection of a range of elements in a smash poset product.
        M_TakeRange(M_TakeRange),
        /// Threshold map (r-to-f for DP\_FuncNotMoreThan)
        M_Threshold1(M_Threshold1),
        /// Threshold map (f-to-r for DP\_ResNotLessThan)
        M_Threshold2(M_Threshold2),
        /// Maps bottom to bottom, and everything else to top.
        M_TopIfNotBottom(M_TopIfNotBottom),
        /// Undefined map
        M_Undefined(M_Undefined),
        /// Placeholder for an unknown map
        M_Unknown(M_Unknown),
        /// Unlifts a one-element tuple to its single element.
        M_Unlift(M_Unlift),
        /// Tests value LEQ constant.
        M_X_Leq_C(M_X_Leq_C),
        /// Tests value LT constant.
        M_X_Lt_C(M_X_Lt_C),
    }
    /// Named DPs represent a graph of DPs with named nodes and node ports.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum NDP {
        /// Graph of NDPs with connections between them.
        NDP_Composite(NDP_Composite),
        /// An NDP that contains a single DP.
        NDP_Simple(NDP_Simple),
        /// sum of NDPs
        NDP_Sum(NDP_Sum),
        /// A special NDP to indicate a template hole in the NDP.
        NDP_TemplateHole(NDP_TemplateHole),
    }
    /// The interface of a named DP.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum NDPInterface {
        /// The interface of a named DP, given by two dictionaries for functionalities and resources.
        NDPInterface_Explicit(NDPInterface_Explicit),
    }
    /// The interface of a named DP, given by two dictionaries for functionalities and resources.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NDPInterface_Explicit {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// Dictionary from functionality name to poset.
        pub fs: std::collections::HashMap<String, Box<Poset>>,
        /// Dictionary from requirement name to poset.
        pub rs: std::collections::HashMap<String, Box<Poset>>,
    }
    /// A template for an NDP.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum NDPTemplate {
        /// A template described by a graph with holes.
        NDPTemplate_Simple(NDPTemplate_Simple),
    }
    /// A template described by a graph with holes.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NDPTemplate_Simple {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub ndp: Box<NDP>,
        /// The interface of the holes.
        pub parameters: std::collections::HashMap<String, Box<NDPInterface>>,
    }
    /// Graph of NDPs with connections between them.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NDP_Composite {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub C: Box<Poset>,
        /// Dictionary of functionalities.
        pub F: std::collections::HashMap<String, Box<Poset>>,
        pub J: Box<Poset>,
        /// Dictionary of resources.
        pub R: std::collections::HashMap<String, Box<Poset>>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// Connections between the nodes in the NDP graph.
        pub connections: Vec<Box<Connection>>,
        /** A map of node identifiers to their corresponding NDPs in the graph.
Each key is a unique identifier for a node, and the value is the NDP
associated with that node.*/
        pub nodes: std::collections::HashMap<String, Box<NDP>>,
    }
    /// An NDP that contains a single DP.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NDP_Simple {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub C: Box<Poset>,
        /// Dictionary of functionalities.
        pub F: std::collections::HashMap<String, Box<Poset>>,
        pub J: Box<Poset>,
        /// Dictionary of resources.
        pub R: std::collections::HashMap<String, Box<Poset>>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The DP that this NDP contains. Must have poset products as resources and functionalities.
        pub dp: Box<DP>,
    }
    /// sum of NDPs
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NDP_Sum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub C: Box<Poset>,
        /// Dictionary of functionalities.
        pub F: std::collections::HashMap<String, Box<Poset>>,
        pub J: Box<Poset>,
        /// Dictionary of resources.
        pub R: std::collections::HashMap<String, Box<Poset>>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The NDPs to sum.
        pub dps: std::collections::HashMap<String, Box<NDP>>,
        /// Labels for the NDPs.
        pub labels: Option<Vec<String>>,
    }
    /// A special NDP to indicate a template hole in the NDP.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NDP_TemplateHole {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub C: Box<Poset>,
        /// Dictionary of functionalities.
        pub F: std::collections::HashMap<String, Box<Poset>>,
        pub J: Box<Poset>,
        /// Dictionary of resources.
        pub R: std::collections::HashMap<String, Box<Poset>>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The name of the parameter that is to be filled in.
        pub parameter_name: String,
    }
    /// The target is the functionality of another subproblem.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeFunctionality {
        pub node: String,
        pub node_functionality: String,
    }
    /// The source of a connection is a requirement of another node.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeRequirement {
        pub node: String,
        pub node_requirement: String,
    }
    /// The poset of boolean values
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_Bool {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// Arrow constructors for posets.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Arrow {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The base poset.
        pub poset: Box<Poset>,
    }
    /// Discretized version of a poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Discretized {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The base poset.
        pub poset: Box<Poset>,
    }
    /// Lexicographic product of posets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Lexicographic {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of labels for the posets.
        pub labels: Option<Vec<String>>,
        /// A list of posets that are composed together.
        pub subs: Vec<Box<Poset>>,
    }
    /// The poset of lower sets of a given poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_LowerSets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The base poset.
        pub poset: Box<Poset>,
    }
    /// Poset of multisets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Multisets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// Poset describing the counts of the elements in the multisets.
        pub counts: Box<Poset>,
        /// Poset describing the values of the multisets.
        pub values: Box<Poset>,
    }
    /// Opposite of a poset
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Opposite {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The base poset.
        pub poset: Box<Poset>,
    }
    /// Power poset of a given poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Power {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The base poset.
        pub poset: Box<Poset>,
    }
    /// Cartesian product of posets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Product {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of labels for the posets.
        pub labels: Option<Vec<String>>,
        /// A list of posets that are composed together.
        pub subs: Vec<Box<Poset>>,
    }
    /// A product of posets where the elements are dictionaries.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_ProductDS {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of labels for the posets.
        pub labels: Option<Vec<String>>,
        /// A list of posets that are composed together.
        pub subs: Vec<Box<Poset>>,
    }
    /// Poset smash product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_ProductSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of labels for the posets.
        pub labels: Option<Vec<String>>,
        /// Whether each poset is "naked" or not.
        pub naked: Vec<bool>,
        /// The ranges of the posets in the smash product. See also P_C_Sum
        pub ranges: Vec<Box<Range>>,
        /// A list of posets that are composed together.
        pub subs: Vec<Box<Poset>>,
    }
    /// Direct sum of posets.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Sum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of labels for the posets.
        pub labels: Option<Vec<String>>,
        /// A list of posets that are composed together.
        pub subs: Vec<Box<Poset>>,
    }
    /// Direct (smash) sum of posets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_SumSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// A list of labels for the posets.
        pub labels: Option<Vec<String>>,
        pub naked: Vec<bool>,
        pub ranges: Vec<Box<Range>>,
        /// A list of posets that are composed together.
        pub subs: Vec<Box<Poset>>,
        pub trivial: bool,
    }
    /// Twisted arrow construction of a poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Twisted {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The base poset.
        pub poset: Box<Poset>,
    }
    /// A poset with units
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_Units {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The base poset.
        pub poset: Box<Poset>,
        /// The units of the poset.
        pub units: Box<Unit>,
    }
    /// The poset of upper sets of a given poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_C_UpperSets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The base poset.
        pub poset: Box<Poset>,
    }
    /// Decimal numbers with fixed precision.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_Decimal {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// Number of decimal places.
        pub precision: i64,
    }
    /// A subposet that allows to sample a numeric poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_F_Bounded {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub bottom: AnyValue,
        pub bound_high: AnyValue,
        pub bound_low: AnyValue,
        pub offset: AnyValue,
        /// The ambient poset.
        pub poset: Box<Poset>,
        pub step: String,
        pub top: AnyValue,
    }
    /// Intersection of posets.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_F_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The ambient poset that includes the others.
        pub ambient: Box<Poset>,
        /// Labels for the posets.
        pub labels: Option<Vec<String>>,
        /** The posets that are included in the intersection. They are all subposets
of the ambient poset.*/
        pub subs: Vec<Box<Poset>>,
    }
    /// Union of posets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_F_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The ambient poset that includes the others.
        pub ambient: Box<Poset>,
        /// Labels for the posets.
        pub labels: Option<Vec<String>>,
        /** The posets that are included in the union. They are all subposets
of the ambient poset.*/
        pub subs: Vec<Box<Poset>>,
    }
    /// An interval in a poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_F_Interval {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The upper bound of the interval.
        pub high: AnyValue,
        /// The lower bound of the interval.
        pub low: AnyValue,
        /// The ambient poset.
        pub poset: Box<Poset>,
    }
    /// Lower closure in a poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_F_LowerClosure {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The lower set.
        pub ls: Box<LowerSet>,
        /// The ambient poset.
        pub poset: Box<Poset>,
    }
    /// A finite subposet of an ambient poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_F_Subposet {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The elements of the subposet.
        pub elements: Vec<AnyValue>,
        /// The ambient poset that contains the elements.
        pub poset: Box<Poset>,
    }
    /// Upper closure in a poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_F_UpperClosure {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// The ambient poset.
        pub poset: Box<Poset>,
        /// The upper set.
        pub us: Box<UpperSet>,
    }
    /// Arbitrary finite poset
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_Finite {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /** Aliases for the elements of the poset.
The keys are the aliases, and the values are arrays of elements that are equivalent to the alias.*/
        pub aliases: Option<std::collections::HashMap<String, Vec<String>>>,
        /// The elements of the poset, strings.
        pub elements: Vec<String>,
        /** The relations of the poset, each relation is a pair of elements.
The first element is less than the second element.*/
        pub relations: Vec<Vec<String>>,
    }
    /// Poset of floating point numbers.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_Float {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /** Precision of the floating point number.
Current supported values are f32 and f64.*/
        pub size: String,
    }
    /// Fractions with a maximum absolute value for numerator and denominator.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_Fractions {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// Maximum absolute value for the denominator.
        pub max_abs_denominator: i64,
        /// Maximum absolute value for the numerator.
        pub max_abs_numerator: i64,
        /// Precision of the fraction.
        pub size: String,
    }
    /// Poset of integers.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_Integer {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        /// Bit size of the integer.
        pub size: String,
    }
    /// Placeholder for an unknown poset
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct P_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
    }
    /// A poset.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum Poset {
        /// The poset of boolean values
        P_Bool(P_Bool),
        /// Arrow constructors for posets.
        P_C_Arrow(P_C_Arrow),
        /// Discretized version of a poset.
        P_C_Discretized(P_C_Discretized),
        /// Lexicographic product of posets
        P_C_Lexicographic(P_C_Lexicographic),
        /// The poset of lower sets of a given poset.
        P_C_LowerSets(P_C_LowerSets),
        /// Poset of multisets
        P_C_Multisets(P_C_Multisets),
        /// Opposite of a poset
        P_C_Opposite(P_C_Opposite),
        /// Power poset of a given poset.
        P_C_Power(P_C_Power),
        /// Cartesian product of posets
        P_C_Product(P_C_Product),
        /// A product of posets where the elements are dictionaries.
        P_C_ProductDS(P_C_ProductDS),
        /// Poset smash product
        P_C_ProductSmash(P_C_ProductSmash),
        /// Direct sum of posets.
        P_C_Sum(P_C_Sum),
        /// Direct (smash) sum of posets
        P_C_SumSmash(P_C_SumSmash),
        /// Twisted arrow construction of a poset.
        P_C_Twisted(P_C_Twisted),
        /// A poset with units
        P_C_Units(P_C_Units),
        /// The poset of upper sets of a given poset.
        P_C_UpperSets(P_C_UpperSets),
        /// Decimal numbers with fixed precision.
        P_Decimal(P_Decimal),
        /// A subposet that allows to sample a numeric poset.
        P_F_Bounded(P_F_Bounded),
        /// Intersection of posets.
        P_F_C_Intersection(P_F_C_Intersection),
        /// Union of posets
        P_F_C_Union(P_F_C_Union),
        /// An interval in a poset.
        P_F_Interval(P_F_Interval),
        /// Lower closure in a poset.
        P_F_LowerClosure(P_F_LowerClosure),
        /// A finite subposet of an ambient poset.
        P_F_Subposet(P_F_Subposet),
        /// Upper closure in a poset.
        P_F_UpperClosure(P_F_UpperClosure),
        /// Arbitrary finite poset
        P_Finite(P_Finite),
        /// Poset of floating point numbers.
        P_Float(P_Float),
        /// Fractions with a maximum absolute value for numerator and denominator.
        P_Fractions(P_Fractions),
        /// Poset of integers.
        P_Integer(P_Integer),
        /// Placeholder for an unknown poset
        P_Unknown(P_Unknown),
    }
    /// Projection from a product.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Projection {
        /// The index of the element to project.
        pub index: i64,
        /// The total number of elements in the product.
        pub ntot: i64,
        /// Type marker
        pub r#type: String,
    }
    /// Queries
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum Query {
        /// Single query
        Query_Single(Query_Single),
    }
    /// Query data
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum QueryData {
        /// Data for the query `FixFunMinReq`
        QueryFixFunMinReqData(QueryFixFunMinReqData),
        /// Data for the query `FixReqMaxFun`
        QueryFixReqMaxFunData(QueryFixReqMaxFunData),
    }
    /// Data for the query `FixFunMinReq`
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryFixFunMinReqData {
        pub f: std::collections::HashMap<String, Box<Value>>,
        pub optimize_for: Vec<String>,
        pub r: std::collections::HashMap<String, Box<Value>>,
    }
    /// Data for the query `FixReqMaxFun`
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryFixReqMaxFunData {
        pub f: std::collections::HashMap<String, Box<Value>>,
        pub optimize_for: Vec<String>,
        pub r: std::collections::HashMap<String, Box<Value>>,
    }
    /// Single query
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Query_Single {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub model: Box<NDP>,
        pub query_data: Box<QueryData>,
    }
    /// Description of a range of integers.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Range {
        /// Total number of elements in the array.
        pub ntot: i64,
        /// Start of the range (inclusive).
        pub start: i64,
        /// End of the range (exclusive).
        pub stop: i64,
        /// Type marker
        pub r#type: String,
    }
    /** Top-level object types for what can be serialized in a file.

 The Root schema contains as subtypes all kinds of objects that can serialized in a MCDP file during an export operation.*/
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "kind")]
    pub enum Root {
        /// Checks for the maps, as used in test cases.
        Check(Check),
        ComputeJob(ComputeJob),
        /// Design problem with implementations (DPI)
        DP(DP),
        /// Map to lower sets of functionalities.
        L1Map(L1Map),
        /// Map to lower sets of functionalities and implementations.
        LMap(LMap),
        /// Monotone maps
        MonotoneMap(MonotoneMap),
        /// Named DPs represent a graph of DPs with named nodes and node ports.
        NDP(NDP),
        /// The interface of a named DP.
        NDPInterface(NDPInterface),
        /// A template for an NDP.
        NDPTemplate(NDPTemplate),
        /// A poset.
        Poset(Poset),
        /// Queries
        Query(Query),
        /// Scalable map to lower sets of functionalities.
        SL1Map(SL1Map),
        /// Scalable map to lower sets of functionalities and implementations.
        SLMap(SLMap),
        /// Scalable map to upper sets of resources.
        SU1Map(SU1Map),
        /// Scalable map to upper sets of resources and implementations.
        SUMap(SUMap),
        /// Map to upper sets of resources.
        U1Map(U1Map),
        /// Map to upper sets of resources and implementations.
        UMap(UMap),
        /// A typed value
        Value(Value),
    }
    /// Check for a SL1Map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1Check {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Test pairs
        pub data: Vec<Box<SL1Check_Data>>,
        /// The map to check
        pub m: Box<SL1Map>,
    }
    /// An input-output pair
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1Check_Data {
        pub opt: AnyValue,
        /// Time taken for the check in seconds.
        pub opt_elapsed: Option<f64>,
        pub opt_y: Box<LowerSet>,
        pub pess: AnyValue,
        /// Time taken for the check in seconds.
        pub pess_elapsed: Option<f64>,
        pub pess_y: Box<LowerSet>,
        pub x: AnyValue,
    }
    /// Scalable map to lower sets of functionalities.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum SL1Map {
        /// Sum of maps
        SL1_C_CodSum(SL1_C_CodSum),
        /// Smash sum
        SL1_C_CodSumSmash(SL1_C_CodSumSmash),
        /// Constructs a SL1Map from explicit approximations of L1Map maps.
        SL1_C_ExplicitApprox(SL1_C_ExplicitApprox),
        /// Intersection of SL1 maps
        SL1_C_Intersection(SL1_C_Intersection),
        /// Monoidal product
        SL1_C_Parallel(SL1_C_Parallel),
        /// Product of domains, intersection of codomains
        SL1_C_ProdIntersection(SL1_C_ProdIntersection),
        /// Product of SL1 maps
        SL1_C_Product(SL1_C_Product),
        /// Refinement of the domain
        SL1_C_RefineDomain(SL1_C_RefineDomain),
        /// Series composition
        SL1_C_Series(SL1_C_Series),
        /// Trace
        SL1_C_Trace(SL1_C_Trace),
        /// Union of SL1 maps
        SL1_C_Union(SL1_C_Union),
        /// Decorates a map with units for the domain and codomain.
        SL1_C_WrapUnits(SL1_C_WrapUnits),
        /// Lifts a L1Map to a SL1Map.
        SL1_Exact(SL1_Exact),
        /// Identity
        SL1_Identity(SL1_Identity),
        /// The lower inverse of multiplication.
        SL1_InvMultiply(SL1_InvMultiply),
        /// The lower inverse of addition.
        SL1_InvSum(SL1_InvSum),
        /// Placeholder for an unknown SL1Map
        SL1_Unknown(SL1_Unknown),
    }
    /// Sum of maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_CodSum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SL1Map>>,
    }
    /// Smash sum
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_CodSumSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SL1Map>>,
    }
    /// Constructs a SL1Map from explicit approximations of L1Map maps.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_ExplicitApprox {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The optimistic approximations of the L1Map.
        pub optimistic: Vec<Box<L1Map>>,
        /// Labels for the optimistic approximations.
        pub optimistic_labels: Option<Vec<String>>,
        /// The pessimistic approximations of the L1Map.
        pub pessimistic: Vec<Box<L1Map>>,
        /// Labels for the pessimistic approximations.
        pub pessimistic_labels: Option<Vec<String>>,
    }
    /// Intersection of SL1 maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SL1Map>>,
    }
    /// Monoidal product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SL1Map>>,
    }
    /// Product of domains, intersection of codomains
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_ProdIntersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SL1Map>>,
    }
    /// Product of SL1 maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_Product {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SL1Map>>,
    }
    /// Refinement of the domain
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        pub m: Box<SL1Map>,
    }
    /// Series composition
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SL1Map>>,
    }
    /// Trace
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        pub m: Box<SL1Map>,
    }
    /// Union of SL1 maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SL1Map>>,
    }
    /// Decorates a map with units for the domain and codomain.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The units for the codomain.
        pub kcod_units: Box<Unit>,
        /// The units for the domain.
        pub kdom_units: Box<Unit>,
        pub m: Box<SL1Map>,
    }
    /// Lifts a L1Map to a SL1Map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_Exact {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The L1Map to lift.
        pub m: Box<L1Map>,
    }
    /// Identity
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
    }
    /// The lower inverse of multiplication.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_InvMultiply {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The poset where the operation takes place
        pub opspace: Box<Poset>,
    }
    /// The lower inverse of addition.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_InvSum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The poset where the operation takes place
        pub opspace: Box<Poset>,
    }
    /// Placeholder for an unknown SL1Map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL1_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
    }
    /// Check for a SL1Map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SLCheck {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Test pairs
        pub data: Vec<Box<SLCheck_Data>>,
        /// The map to check
        pub m: Box<SLMap>,
    }
    /// An input-output pair for the SLMap
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SLCheck_Data {
        pub opt: AnyValue,
        /// Time taken for the check in seconds.
        pub opt_elapsed: Option<f64>,
        pub opt_y: Box<LowerSet>,
        pub pess: AnyValue,
        /// Time taken for the check in seconds.
        pub pess_elapsed: Option<f64>,
        pub pess_y: Box<LowerSet>,
        pub x: AnyValue,
    }
    /// Scalable map to lower sets of functionalities and implementations.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum SLMap {
        /// Transforms the implementations of a SLMap.
        SL_C_ITransform(SL_C_ITransform),
        /// Intersection of the results of a set of maps.
        SL_C_Intersection(SL_C_Intersection),
        /// Monoidal product
        SL_C_Parallel(SL_C_Parallel),
        /// Refines the domain of another SLMap
        SL_C_RefineDomain(SL_C_RefineDomain),
        /// Series composition
        SL_C_Series(SL_C_Series),
        /// Trace of a SLMap.
        SL_C_Trace(SL_C_Trace),
        /// Trace of a SLMap (second version)
        SL_C_TraceL(SL_C_TraceL),
        /// Composition of SLMaps using the union of the results.
        SL_C_Union(SL_C_Union),
        /// Decorates with units another SLMap.
        SL_C_WrapUnits(SL_C_WrapUnits),
        /// Identity
        SL_Identity(SL_Identity),
        /// Lifts a LMap to a SLMap.
        SL_L_Exact(SL_L_Exact),
        /// Construct a SLMap from explicit optimistic and pessimistic approximations.
        SL_L_Explicit_Approx(SL_L_Explicit_Approx),
        /// Lifts a SL1Map to SLMap with a constant implementation.
        SL_L_Lift1_Constant(SL_L_Lift1_Constant),
        /// Lifts a SL1Map to SLMap by generating the implementations.
        SL_L_Lift1_Transform(SL_L_Lift1_Transform),
        /// Placeholder for unknown SLMap
        SL_Unknown(SL_Unknown),
    }
    /// Transforms the implementations of a SLMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_ITransform {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        pub m: Box<SLMap>,
        pub transform: Box<MonotoneMap>,
    }
    /// Intersection of the results of a set of maps.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<SLMap>>,
    }
    /// Monoidal product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<SLMap>>,
    }
    /// Refines the domain of another SLMap
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        pub m: Box<SLMap>,
    }
    /// Series composition
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<SLMap>>,
    }
    /// Trace of a SLMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        pub m: Box<SLMap>,
        pub m_proj: Box<SL1Map>,
    }
    /// Trace of a SLMap (second version)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_TraceL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        pub m: Box<SLMap>,
        pub m_proj: Box<SL1Map>,
    }
    /// Composition of SLMaps using the union of the results.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<SLMap>>,
    }
    /// Decorates with units another SLMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// Units for the codomain of the SLMap.
        pub kcod_units: Box<Unit>,
        /// Units for the domain of the SLMap.
        pub kdom_units: Box<Unit>,
        /// Units for the implementations of the SLMap.
        pub kimp_units: Box<Unit>,
        pub m: Box<SLMap>,
    }
    /// Identity
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
    }
    /// Lifts a LMap to a SLMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_L_Exact {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// The LMap to be lifted to a SLMap.
        pub m: Box<LMap>,
    }
    /// Construct a SLMap from explicit optimistic and pessimistic approximations.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_L_Explicit_Approx {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// The optimistic approximations of the SLMap.
        pub optimistic: Vec<Box<LMap>>,
        /// Labels for the optimistic approximations.
        pub optimistic_labels: Option<Vec<String>>,
        /// The pessimistic approximations of the SLMap.
        pub pessimistic: Vec<Box<LMap>>,
        /// Labels for the pessimistic approximations.
        pub pessimistic_labels: Option<Vec<String>>,
    }
    /// Lifts a SL1Map to SLMap with a constant implementation.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_L_Lift1_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// The SL1Map to be lifted to a SLMap.
        pub m: Box<SL1Map>,
        /// The constant value to be used for the implementations
        pub value: AnyValue,
    }
    /// Lifts a SL1Map to SLMap by generating the implementations.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_L_Lift1_Transform {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// The SL1Map to be lifted to a SLMap.
        pub m: Box<SL1Map>,
        /// The monotone map that transforms the implementations of the SLMap.
        pub transform: Box<MonotoneMap>,
    }
    /// Placeholder for unknown SLMap
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SL_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
    }
    /// Check for a SU1Map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1Check {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Test pairs
        pub data: Vec<Box<SU1Check_Data>>,
        /// The map to check
        pub m: Box<SU1Map>,
    }
    /// An input-output pair for the SU1Map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1Check_Data {
        pub opt: AnyValue,
        /// Time taken for the check in seconds.
        pub opt_elapsed: Option<f64>,
        pub opt_y: Box<UpperSet>,
        pub pess: AnyValue,
        /// Time taken for the check in seconds.
        pub pess_elapsed: Option<f64>,
        pub pess_y: Box<UpperSet>,
        pub x: AnyValue,
    }
    /// Scalable map to upper sets of resources.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum SU1Map {
        /// Sum of maps
        SU1_C_CodSum(SU1_C_CodSum),
        /// Smash sum
        SU1_C_CodSumSmash(SU1_C_CodSumSmash),
        /// Constructs a SU1Map from explicit approximations of U1Map maps.
        SU1_C_ExplicitApprox(SU1_C_ExplicitApprox),
        /// Intersection of SU1 maps
        SU1_C_Intersection(SU1_C_Intersection),
        /// Monoidal product
        SU1_C_Parallel(SU1_C_Parallel),
        /// Product of domains, intersection of codomains
        SU1_C_ProdIntersection(SU1_C_ProdIntersection),
        /// Product of SU1 maps
        SU1_C_Product(SU1_C_Product),
        /// Refinement of the domain
        SU1_C_RefineDomain(SU1_C_RefineDomain),
        /// Series composition
        SU1_C_Series(SU1_C_Series),
        /// Trace
        SU1_C_Trace(SU1_C_Trace),
        /// Union of SU1 maps
        SU1_C_Union(SU1_C_Union),
        /// Wraps a map with units.
        SU1_C_WrapUnits(SU1_C_WrapUnits),
        /// Lifts a U1Map to a SU1Map.
        SU1_Exact(SU1_Exact),
        /// Identity
        SU1_Identity(SU1_Identity),
        /// The upper inverse of multiplication.
        SU1_InvMultiply(SU1_InvMultiply),
        /// The inverse of addition.
        SU1_InvSum(SU1_InvSum),
        /// Placeholder for an unknown SU1Map
        SU1_Unknown(SU1_Unknown),
    }
    /// Sum of maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_CodSum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SU1Map>>,
    }
    /// Smash sum
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_CodSumSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SU1Map>>,
    }
    /// Constructs a SU1Map from explicit approximations of U1Map maps.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_ExplicitApprox {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The optimistic approximations of the map
        pub optimistic: Vec<Box<U1Map>>,
        /// Labels for the optimistic approximations.
        pub optimistic_labels: Option<Vec<String>>,
        /// The pessimistic approximations of the map
        pub pessimistic: Vec<Box<U1Map>>,
        /// Labels for the pessimistic approximations.
        pub pessimistic_labels: Option<Vec<String>>,
    }
    /// Intersection of SU1 maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SU1Map>>,
    }
    /// Monoidal product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SU1Map>>,
    }
    /// Product of domains, intersection of codomains
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_ProdIntersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SU1Map>>,
    }
    /// Product of SU1 maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_Product {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SU1Map>>,
    }
    /// Refinement of the domain
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        pub m: Box<SU1Map>,
    }
    /// Series composition
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SU1Map>>,
    }
    /// Trace
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        pub m: Box<SU1Map>,
    }
    /// Union of SU1 maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// A list of labels for the maps.
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SU1Map>>,
    }
    /// Wraps a map with units.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// Units for the codomain
        pub kcod_units: Box<Unit>,
        /// Units for the domain
        pub kdom_units: Box<Unit>,
        pub m: Box<SU1Map>,
    }
    /// Lifts a U1Map to a SU1Map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_Exact {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The U1Map to be lifted to a SU1Map.
        pub m: Box<U1Map>,
    }
    /// Identity
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
    }
    /// The upper inverse of multiplication.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_InvMultiply {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The poset where the operation is defined.
        pub opspace: Box<Poset>,
    }
    /// The inverse of addition.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_InvSum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
        /// The poset where the operation is defined.
        pub opspace: Box<Poset>,
    }
    /// Placeholder for an unknown SU1Map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU1_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub opt: Box<Poset>,
        pub pes: Box<Poset>,
    }
    /// Check for a SUMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SUCheck {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Test pairs
        pub data: Vec<Box<SUCheck_Data>>,
        /// The map to check
        pub m: Box<SUMap>,
    }
    /// An input-output pair for the SUMap
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SUCheck_Data {
        pub opt: AnyValue,
        /// Time taken for the check in seconds.
        pub opt_elapsed: Option<f64>,
        pub opt_y: Box<UpperSet>,
        pub pess: AnyValue,
        /// Time taken for the check in seconds.
        pub pess_elapsed: Option<f64>,
        pub pess_y: Box<UpperSet>,
        pub x: AnyValue,
    }
    /// Scalable map to upper sets of resources and implementations.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum SUMap {
        /// Transforms the implementations of a SUMap.
        SU_C_ITransform(SU_C_ITransform),
        /// Intersection of the results of a set of maps.
        SU_C_Intersection(SU_C_Intersection),
        /// Monoidal product
        SU_C_Parallel(SU_C_Parallel),
        /// Refines the domain of another SUMap
        SU_C_RefineDomain(SU_C_RefineDomain),
        /// Series composition
        SU_C_Series(SU_C_Series),
        /// Trace of a SUMap.
        SU_C_Trace(SU_C_Trace),
        /// Trace of a SUMap (second version)
        SU_C_TraceL(SU_C_TraceL),
        /// Composition of SUMaps using the union of the results.
        SU_C_Union(SU_C_Union),
        /// Decorates with units another SUMap.
        SU_C_WrapUnits(SU_C_WrapUnits),
        /// Identity
        SU_Identity(SU_Identity),
        /// Lifts a UMap to a SUMap.
        SU_L_Exact(SU_L_Exact),
        /// Construct a SUMap from explicit optimistic and pessimistic approximations.
        SU_L_Explicit_Approx(SU_L_Explicit_Approx),
        /// Lifts a SU1Map to SUMap with a constant implementation.
        SU_L_Lift1_Constant(SU_L_Lift1_Constant),
        /// Lifts a SU1Map to SUMap by generating the implementations.
        SU_L_Lift1_Transform(SU_L_Lift1_Transform),
        /// Placeholder for unknown SUMap
        SU_Unknown(SU_Unknown),
    }
    /// Transforms the implementations of a SUMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_ITransform {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        pub m: Box<SUMap>,
        pub transform: Box<MonotoneMap>,
    }
    /// Intersection of the results of a set of maps.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// Labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SUMap>>,
    }
    /// Monoidal product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// Labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SUMap>>,
    }
    /// Refines the domain of another SUMap
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        pub m: Box<SUMap>,
    }
    /// Series composition
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// Labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SUMap>>,
    }
    /// Trace of a SUMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        pub m: Box<SUMap>,
        pub m_proj: Box<SU1Map>,
    }
    /// Trace of a SUMap (second version)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_TraceL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        pub m: Box<SUMap>,
        pub m_proj: Box<SU1Map>,
    }
    /// Composition of SUMaps using the union of the results.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// Labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<SUMap>>,
    }
    /// Decorates with units another SUMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// Units for the codomain of the SUMap.
        pub kcod_units: Box<Unit>,
        /// Units for the domain of the SUMap.
        pub kdom_units: Box<Unit>,
        /// Units for the implementations of the SUMap.
        pub kimp_units: Box<Unit>,
        pub m: Box<SUMap>,
    }
    /// Identity
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
    }
    /// Lifts a UMap to a SUMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_L_Exact {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// The UMap to be lifted to a SUMap.
        pub m: Box<UMap>,
    }
    /// Construct a SUMap from explicit optimistic and pessimistic approximations.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_L_Explicit_Approx {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// The optimistic approximations of the SUMap.
        pub optimistic: Vec<Box<UMap>>,
        /// Labels for the optimistic approximations.
        pub optimistic_labels: Option<Vec<String>>,
        /// The pessimistic approximations of the SUMap.
        pub pessimistic: Vec<Box<UMap>>,
        /// Labels for the pessimistic approximations.
        pub pessimistic_labels: Option<Vec<String>>,
    }
    /// Lifts a SU1Map to SUMap with a constant implementation.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_L_Lift1_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// The SU1Map to be lifted to a SUMap.
        pub m: Box<SU1Map>,
        /// The constant value to be used for the implementations
        pub value: AnyValue,
    }
    /// Lifts a SU1Map to SUMap by generating the implementations.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_L_Lift1_Transform {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
        /// The SU1Map to be lifted to a SUMap.
        pub m: Box<SU1Map>,
        /// The monotone map that transforms the implementations of the SUMap.
        pub transform: Box<MonotoneMap>,
    }
    /// Placeholder for unknown SUMap
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SU_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Kleisli co-domain of the map.
        pub kcod: Box<Poset>,
        /// Kleisli domain of the map.
        pub kdom: Box<Poset>,
        /// Poset of implementations.
        pub kimp: Box<Poset>,
        /// Poset of resolutions (optimistic)
        pub opt: Box<Poset>,
        /// Poset of resolutions (pessimistic)
        pub pes: Box<Poset>,
    }
    /// Check for a U1Map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1Check {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Test pairs
        pub data: Vec<Box<U1Check_Data>>,
        /// The map to check
        pub m: Box<U1Map>,
    }
    /// An input-output pair for the U1Map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1Check_Data {
        /// Time taken for the check in seconds.
        pub elapsed: Option<f64>,
        pub x: AnyValue,
        pub y: Box<UpperSet>,
    }
    /// Map to upper sets of resources.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum U1Map {
        /// Co-domain sum combination
        U1_C_CodSum(U1_C_CodSum),
        /// Co-domain (smash) sum combination
        U1_C_CodSumSmash(U1_C_CodSumSmash),
        /// Domain union
        U1_C_DomUnion(U1_C_DomUnion),
        /// Intersection
        U1_C_Intersection(U1_C_Intersection),
        /// Monoidal product
        U1_C_Parallel(U1_C_Parallel),
        /// From product to intersection
        U1_C_ProdIntersection(U1_C_ProdIntersection),
        /// Product
        U1_C_Product(U1_C_Product),
        /// Refines the domain of a monotone map.
        U1_C_RefineDomain(U1_C_RefineDomain),
        /// Series composition
        U1_C_Series(U1_C_Series),
        /// Trace
        U1_C_Trace(U1_C_Trace),
        /// Union
        U1_C_Union(U1_C_Union),
        /// Decorates a map with units.
        U1_C_WrapUnits(U1_C_WrapUnits),
        /// Map induced by a catalog of options.
        U1_Catalog(U1_Catalog),
        /// Constant map
        U1_Constant(U1_Constant),
        /// Returns the entire poset
        U1_Entire(U1_Entire),
        /// Map defined pointwise
        U1_Explicit(U1_Explicit),
        /// Filters based on a monotone map.
        U1_FromFilter(U1_FromFilter),
        /// Lift of the identity map
        U1_Identity(U1_Identity),
        /// Intersection of principal upper sets.
        U1_IntersectionOfPrinUpperSets(U1_IntersectionOfPrinUpperSets),
        /// Finite-resolution optimistic approximation of the inverse of a multiplication map.
        U1_InvMul_Opt(U1_InvMul_Opt),
        /// Finite-resolution pessimistic approximation of the inverse of a multiplication map.
        U1_InvMul_Pes(U1_InvMul_Pes),
        /// Finite-resolution optimistic approximation of the inverse of an addition map.
        U1_InvSum_Opt(U1_InvSum_Opt),
        /// Finite-resolution pessimistic approximation of the inverse of an addition map.
        U1_InvSum_Pes(U1_InvSum_Pes),
        /// Computes the upper inverse of a monotone map.
        U1_L_Uinv(U1_L_Uinv),
        /// Lifts a monotone map
        U1_Lift(U1_Lift),
        /// Represent a principal upper set
        U1_RepresentPrincipalUpperSet(U1_RepresentPrincipalUpperSet),
        U1_Uinv_Join(U1_Uinv_Join),
        U1_Uinv_JoinConstant(U1_Uinv_JoinConstant),
        /// Union of principal upper sets.
        U1_UnionOfPrinUpperSets(U1_UnionOfPrinUpperSets),
        /// Placeholder for an unknown map.
        U1_Unknown(U1_Unknown),
    }
    /// Co-domain sum combination
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_CodSum {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// Co-domain (smash) sum combination
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_CodSumSmash {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// Domain union
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_DomUnion {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// Intersection
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// Monoidal product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// From product to intersection
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_ProdIntersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// Product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_Product {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// Refines the domain of a monotone map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<U1Map>,
    }
    /// Series composition
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// Trace
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<U1Map>,
    }
    /// Union
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// A list of labels for the maps
        pub labels: Option<Vec<String>>,
        pub ms: Vec<Box<U1Map>>,
    }
    /// Decorates a map with units.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Units for the codomain
        pub kcod_units: Box<Unit>,
        /// Units for the domain
        pub kdom_units: Box<Unit>,
        pub m: Box<U1Map>,
    }
    /// Map induced by a catalog of options.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Catalog {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub options: Vec<Box<U1_Catalog_Options>>,
    }
    /// An option in the catalog
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Catalog_Options {
        pub f: AnyValue,
        pub r: AnyValue,
    }
    /// Constant map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub value: Box<UpperSet>,
    }
    /// Returns the entire poset
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Entire {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Map defined pointwise
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Explicit {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub options: Vec<Box<U1_Explicit_Option>>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Explicit_Option {
        pub x: AnyValue,
        pub y: Box<UpperSet>,
    }
    /// Filters based on a monotone map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_FromFilter {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<MonotoneMap>,
    }
    /// Lift of the identity map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Intersection of principal upper sets.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_IntersectionOfPrinUpperSets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Finite-resolution optimistic approximation of the inverse of a multiplication map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_InvMul_Opt {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Resolution (number of points in the produced antichain)
        pub n: i64,
        /// The poset in which the operation is performed.
        pub opspace: Box<Poset>,
    }
    /// Finite-resolution pessimistic approximation of the inverse of a multiplication map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_InvMul_Pes {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Resolution (number of points in the produced antichain)
        pub n: i64,
        /// The poset in which the operation is performed.
        pub opspace: Box<Poset>,
    }
    /// Finite-resolution optimistic approximation of the inverse of an addition map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_InvSum_Opt {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Resolution (number of points in the produced antichain)
        pub n: i64,
        /// The poset in which the operation is performed.
        pub opspace: Box<Poset>,
    }
    /// Finite-resolution pessimistic approximation of the inverse of an addition map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_InvSum_Pes {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        /// Resolution (number of points in the produced antichain)
        pub n: i64,
        /// The poset in which the operation is performed.
        pub opspace: Box<Poset>,
    }
    /// Computes the upper inverse of a monotone map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_L_Uinv {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<MonotoneMap>,
    }
    /// Lifts a monotone map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Lift {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub m: Box<MonotoneMap>,
    }
    /// Represent a principal upper set
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_RepresentPrincipalUpperSet {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Uinv_Join {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub lower_bounds: Vec<Vec<AnyValue>>,
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Uinv_JoinConstant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub join1_dom: Box<Poset>,
        pub value: Box<Value>,
    }
    /// Union of principal upper sets.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_UnionOfPrinUpperSets {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Placeholder for an unknown map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U1_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
    }
    /// Check for a UMap.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UCheck {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Test pairs
        pub data: Vec<Box<UCheck_Data>>,
        /// The map to check
        pub m: Box<UMap>,
    }
    /// An input-output pair for the UMap
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UCheck_Data {
        /// Time taken for the check in seconds.
        pub elapsed: Option<f64>,
        pub x: AnyValue,
        pub y: Box<UpperSet>,
    }
    /// Map to upper sets of resources and implementations.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum UMap {
        /// Transforms the implementation of another map.
        U_C_ITransform(U_C_ITransform),
        /// Intersection of maps
        U_C_Intersection(U_C_Intersection),
        /// Monoidal product
        U_C_Parallel(U_C_Parallel),
        /// Refines the domain of a monotone map
        U_C_RefineDomain(U_C_RefineDomain),
        /// Series composition
        U_C_Series(U_C_Series),
        /// Trace
        U_C_Trace(U_C_Trace),
        /// Trace (second version with extra imp)
        U_C_TraceL(U_C_TraceL),
        /// Union of maps
        U_C_Union(U_C_Union),
        /// Decorates a map with units.
        U_C_WrapUnits(U_C_WrapUnits),
        /// UMap for a catalog
        U_Catalog(U_Catalog),
        /// Constant map
        U_Constant(U_Constant),
        /// Identity
        U_Identity(U_Identity),
        /// Lifts a U1Map morphism with a constant value for the implementation.
        U_L_Lift1_Constant(U_L_Lift1_Constant),
        /// Lifts a U1Map morphism with a function to compute the implementation.
        U_L_Lift1_Transform(U_L_Lift1_Transform),
        /// Placeholder for an unknown map
        U_Unknown(U_Unknown),
    }
    /// Transforms the implementation of another map.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_ITransform {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<UMap>,
        pub transform: Box<MonotoneMap>,
    }
    /// Intersection of maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_Intersection {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// Labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<UMap>>,
    }
    /// Monoidal product
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_Parallel {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// Labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<UMap>>,
    }
    /// Refines the domain of a monotone map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_RefineDomain {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<UMap>,
    }
    /// Series composition
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_Series {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// Labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<UMap>>,
    }
    /// Trace
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_Trace {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<UMap>,
        pub m_proj: Box<U1Map>,
    }
    /// Trace (second version with extra imp)
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_TraceL {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<UMap>,
        pub m_proj: Box<U1Map>,
    }
    /// Union of maps
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_Union {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// Labels for the maps.
        pub labels: Option<Vec<String>>,
        /// Maps to be composed.
        pub ms: Vec<Box<UMap>>,
    }
    /// Decorates a map with units.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_C_WrapUnits {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub kcod_units: Box<Unit>,
        pub kdom_units: Box<Unit>,
        pub kimp_units: Box<Unit>,
        pub m: Box<UMap>,
    }
    /// UMap for a catalog
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_Catalog {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// The options in the catalog.
        pub options: Vec<Box<U_Catalog_Options>>,
    }
    /// An option in the catalog
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_Catalog_Options {
        pub f: AnyValue,
        pub i: AnyValue,
        pub r: AnyValue,
    }
    /// Constant map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        /// The upper set that is the value of the constant map.
        pub value: Box<UpperSet>,
    }
    /// Identity
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_Identity {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
    }
    /// Lifts a U1Map morphism with a constant value for the implementation.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_L_Lift1_Constant {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<U1Map>,
        pub value: AnyValue,
    }
    /// Lifts a U1Map morphism with a function to compute the implementation.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_L_Lift1_Transform {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
        pub m: Box<U1Map>,
        pub transform: Box<MonotoneMap>,
    }
    /// Placeholder for an unknown map
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct U_Unknown {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        pub kcod: Box<Poset>,
        pub kdom: Box<Poset>,
        pub kimp: Box<Poset>,
    }
    /// Units specifications
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum Unit {
        /// Represents the absence of units.
        Unit_None(Unit_None),
        /// A simple unit.
        Unit_Single(Unit_Single),
        /// A vector of units for a product of posets.
        Unit_Vector(Unit_Vector),
        /// A special type of unit that is used to describe the units of composite types.
        Unit_Wrapped(Unit_Wrapped),
    }
    /// Represents the absence of units.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Unit_None {
        /// A human-readable description of the unit (debug purposes).
        pub description: Option<String>,
        /// Kind marker.
        pub kind: String,
    }
    /// A simple unit.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Unit_Single {
        /// A human-readable description of the unit (debug purposes).
        pub description: Option<String>,
        /// Kind marker.
        pub kind: String,
        /// A string representing the unit.
        pub units: String,
    }
    /// A vector of units for a product of posets.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Unit_Vector {
        /// A human-readable description of the unit (debug purposes).
        pub description: Option<String>,
        /// Kind marker.
        pub kind: String,
        /// labels for the subunits
        pub labels: Option<Vec<String>>,
        /// The subunits.
        pub subs: Vec<Box<Unit>>,
    }
    /// A special type of unit that is used to describe the units of composite types.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Unit_Wrapped {
        /// A human-readable description of the unit (debug purposes).
        pub description: Option<String>,
        /// Kind marker.
        pub kind: String,
        pub inside: Vec<Box<Unit>>,
        pub name: String,
        pub shape: AnyValue,
    }
    /// Upper sets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum UpperSet {
        /// Unused
        UpperSet_Unused(UpperSet_Unused),
        /// An upper set defined as the up closure of a finite set of points.
        UpperSet_UpperClosure(UpperSet_UpperClosure),
    }
    /// Unused
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpperSet_Unused {
        /// Kind marker.
        pub kind: String,
    }
    /// An upper set defined as the up closure of a finite set of points.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpperSet_UpperClosure {
        /// Kind marker.
        pub kind: String,
        /// The points in the set.
        pub points: Vec<AnyValue>,
    }
    /// A (poset, value) pair.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VU {
        /// A human-readable description of the object used for debug purposes.
        pub description: Option<String>,
        /// Unique hash for the object.
        pub hash: Option<String>,
        /// Version of the MCDP format used to serialize this object (major.minor).
        pub version: Option<String>,
        /// Pointer to the entity that generated this object.
        pub address: Option<Box<Address>>,
        pub poset: Box<Poset>,
        pub value: AnyValue,
    }
    /// A typed value
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum Value {
        /// A (poset, value) pair.
        VU(VU),
    }
}
