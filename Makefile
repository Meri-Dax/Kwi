ENV_FILE := $(if $(filter test% %-test,$(MAKECMDGOALS)),.env.test,.env)

-include $(ENV_FILE)
export

.env:
	@if [ -f .env.dist ]; then \
		echo "# Default .env created from .env.dist" > .env; \
		cat .env.dist >> .env; \
		$${EDITOR:-vi} .env; \
	else \
		echo "Error: .env.dist not found!"; exit 1; \
	fi

.env.test:
	@if [ -f .env.dist ]; then \
		echo "# Default .env.test created from .env.dist.\n#‼️ Make sure the specified database is dedicated to testing, it _will_ be wiped. you can specify any name, it will then be auto-generated." > .env.test; \
		cat .env.dist >> .env.test; \
		$${EDITOR:-vi} .env.test; \
	else \
		echo "Error: .env.dist not found!"; exit 1; \
	fi

default: .env
