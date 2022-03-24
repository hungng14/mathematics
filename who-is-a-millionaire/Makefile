DB_URL=postgresql://root:123456@172.22.0.3:5432/who_is_a_millionaire
filename:=
migrate-add:
	cargo sqlx migrate add -r ${filename}
# ex: migrate-add filename=...

migrate-run: 
	cargo sqlx migrate run --database-url $(DB_URL)

migrate-revert: 
	cargo sqlx migrate revert --database-url $(DB_URL)
# remove record in table migration