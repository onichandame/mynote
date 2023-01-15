## Development

All database's schema changes must be implemented in `migration` crate. The post-ops are:

1. `sea migrate -u sqlite:test.db?mode=rwc`
1. `sea generate entity --lib -o entity/src/ -u sqlite:test.db`
1. `rm -f test.db`
