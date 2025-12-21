regen_db:
  sea-orm-cli migrate fresh
  sea-orm-cli generate entity -o db/src/entities --entity-format dense

entities:
  sea-orm-cli generate entity -o db/src/entities --entity-format dense
