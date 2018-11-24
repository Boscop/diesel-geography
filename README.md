# diesel-geography
Diesel support for PostGIS geography types and functions

### How to use it:

In your sql schema, you have a column `location geography(point, 4326) not null`.
When Diesel generates the schema (using `table! {}`) this column will look like `location -> Geography`.
To ensure that the `Geography` type is in scope, read [this guide](http://diesel.rs/guides/configuring-diesel-cli/) and add `use diesel_geography::sql_types::*` to the `import_types` key in your `diesel.toml` file.

E.g. it will look like this:
```toml
[print_schema]
file = "src/schema.rs"

import_types = ["diesel::sql_types::*", "diesel_geography::sql_types::*"]
```

In your ORM struct, write `location: GeogPoint`. Now you can use this struct / table in your diesel queries.
