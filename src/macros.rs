#[macro_export]
/// This macro is used to impl the SelectModel for models
macro_rules! select_model {
    ($struct:ty, $table_name:ident) => {

        #[async_trait]
        impl SelectModel for $struct {
            async fn from_id(conn: DbPool, id: i32) -> Self {
                use $crate::schema::$table_name::dsl as $table_name;

                let mut conn = conn.get().unwrap();

                block(move || {
                    // Get the Required Resource
                    let selected_model: Vec<$struct> = $table_name::$table_name
                        // Filter it by id (select by id)
                        .filter($table_name::id.eq(id))
                        .load(&mut conn)
                        .unwrap();

                    // Get the first item of vec
                    let selected_model = selected_model.get(0).unwrap();

                    selected_model.clone()
                })
                .await
                .unwrap()
            }
        }
    };
}

