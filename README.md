# Pitico
## Introduction
Pitico is a minimal URL shortener. It is a [Rocket](https://github.com/rwf2/Rocket/tree/v0.5) web server that shortens URLs with a Base 62 conversion approach. The shortened URLs are saved in a [rusqlite](https://github.com/rusqlite/rusqlite) local database.

## Usage
### Running Pitico
```
git clone git@github.com:LucasNobrega/Pitico.git
cd Pitico
cargo run
```

### Register/Shorten an URL
Run an HTTP GET request to: `http://<ip>:<port>/register/<url>`. For example:
```
curl -s "http://127.0.0.1:8000/register/youtube.com"
```

### Access shortened URL
Access `http://<ip>:<port>/<pitico_url>` in your browser!

### Fill database
To fill the database with default websites, start Pitico and run:
```
cd utils/fill_db
bash fill_db.sh
```

## Configure
The Rocket server's ip and port can be configured in the Rocket.toml file. The default ip and port are: `127.0.0.1` and `8000`.
