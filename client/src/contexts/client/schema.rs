#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct LoginArguments {
        pub identity: String,
        pub password: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "LoginArguments")]
    pub struct Login {
        #[arguments(input = LoginInput { identity: args.identity.clone(), password: args.password.clone() })]
        pub login: String,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct LoginInput {
        pub identity: String,
        pub password: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct Users {
        pub users: UserConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct UserConnection {
        pub nodes: Vec<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub updated_at: Option<NaiveDateTime>,
        pub email: Option<String>,
        pub created_at: NaiveDateTime,
        pub avatar: Option<String>,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct NaiveDateTime(pub String);
}

mod schema {
    cynic::use_schema!(r#"schema.graphql"#);
}
