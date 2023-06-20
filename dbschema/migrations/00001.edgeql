CREATE MIGRATION m1nxz6zrug5bsx4baf3zx7twyti44qywto3s4m4p2ffklb4enwwwkq
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE TYPE default::BlogCategory {
      CREATE REQUIRED PROPERTY slug -> std::str {
          CREATE CONSTRAINT std::exclusive;
          CREATE CONSTRAINT std::max_len_value(50);
      };
      CREATE INDEX ON (std::str_lower(.slug));
      CREATE PROPERTY old_id -> std::int16 {
          SET readonly := true;
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY title -> std::str {
          CREATE CONSTRAINT std::max_len_value(50);
      };
  };
  CREATE TYPE default::User {
      CREATE REQUIRED PROPERTY email -> std::str {
          CREATE CONSTRAINT std::exclusive;
          CREATE CONSTRAINT std::max_len_value(200);
      };
      CREATE INDEX ON (std::str_lower(.email));
      CREATE REQUIRED PROPERTY username -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE INDEX ON (std::str_lower(.username));
      CREATE PROPERTY first_name -> std::str {
          CREATE CONSTRAINT std::max_len_value(40);
      };
      CREATE PROPERTY is_active -> std::bool {
          SET default := true;
      };
      CREATE PROPERTY is_superuser -> std::bool {
          SET default := false;
      };
      CREATE PROPERTY last_name -> std::str {
          CREATE CONSTRAINT std::max_len_value(40);
      };
      CREATE REQUIRED PROPERTY password -> std::str {
          CREATE CONSTRAINT std::max_len_value(100);
      };
  };
  CREATE SCALAR TYPE default::DocFormat EXTENDING enum<Md, Rst>;
  CREATE TYPE default::BlogPost {
      CREATE MULTI LINK categories -> default::BlogCategory;
      CREATE REQUIRED PROPERTY slug -> std::str {
          CREATE CONSTRAINT std::exclusive;
          CREATE CONSTRAINT std::max_len_value(200);
      };
      CREATE INDEX ON (std::str_lower(.slug));
      CREATE LINK author -> default::User {
          ON TARGET DELETE ALLOW;
      };
      CREATE PROPERTY body -> std::str;
      CREATE PROPERTY created_at -> std::datetime {
          SET default := (std::datetime_current());
          SET readonly := true;
      };
      CREATE PROPERTY excerpt -> std::str;
      CREATE PROPERTY format -> default::DocFormat {
          SET default := (default::DocFormat.Md);
      };
      CREATE PROPERTY html -> std::str;
      CREATE PROPERTY is_published -> std::bool {
          SET default := false;
      };
      CREATE PROPERTY locale -> std::str {
          CREATE CONSTRAINT std::max_len_value(6);
      };
      CREATE PROPERTY og_image -> std::str {
          CREATE CONSTRAINT std::max_len_value(200);
      };
      CREATE PROPERTY old_id -> std::int16 {
          SET readonly := true;
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY published_at -> std::datetime {
          SET default := (std::datetime_current());
      };
      CREATE PROPERTY seo_description -> std::str {
          CREATE CONSTRAINT std::max_len_value(400);
      };
      CREATE MULTI PROPERTY seo_keywords -> std::str {
          CREATE CONSTRAINT std::max_len_value(40);
      };
      CREATE REQUIRED PROPERTY title -> std::str {
          CREATE CONSTRAINT std::max_len_value(200);
      };
      CREATE PROPERTY updated_at -> std::datetime {
          SET default := (std::datetime_current());
      };
  };
};
