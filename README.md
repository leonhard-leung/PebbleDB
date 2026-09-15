# PebbleDB
> A CLI-fist database engine for rapid prototyping and local application development.

## About
PebbleDB is an open-source database engine designed for developers who need a fast and portable solution for local
application development. It simplifies database setup by providing a single executable that can create, manage, and
serve local databases without requiring traditional database servers or containerization tools.

Built around a modular architecture, PebbleDB consists of a command-line interface, parser, database engine, storage
engine, and built-in HTTP server. While the project currently focuses on a CLI-first experience, its long-term goal
is to support multiple interfaces for applications to interact with the database.

PebbleDB is also developed as an educational project, providing an opportunity to explore the design and 
implementation of a database system from the ground up.

## Why PebbleDB?
Modern application development often requires installing and configuring a database before writing application code.
While established database systems are well suited for production environments, they can introduce additional setup
during prototyping, hackathons, classroom activitites, and personal projects.

PebbleDB aims to reduce this setup by providing a local database that can be created, managed, and accessed from a
single application.

> ***Disclaimer:*** PebbleDB is not intended to compete with or replace established database systems such as MySQL,
> PostgreSQL, Microsoft SQL Server, or Oracle Database, and many other enterprise-grade database systems. These
> systems are designed to support production and enterprise workloads and offer capabilities that are beyond the scope
> of PebbleDB. If you are developing an application intended for production, enterprise, or other critical environments
> that require advanced database capabilities, scalability, reliability, security, or high availability, PebbleDB
> is not intended for such use.

## Features
The following table outlines PebbleDB's current capabilities and planned features.

| Status | Feature                       |
|:------:|-------------------------------|
|   ✅   | Interactive CLI               |
|   ✅   | Database creation             |
|   ✅   | Table creation and management |
|   ✅   | CRUD operations               |
|   ✅   | Persistent local storage      |
|   ✅   | Built-in local server         |
|   📅   | Filtering and sorting         |
|   📅   | Constraints                   |
|   📅   | Transactions                  |
|   📅   | Indexing                      |
|   📅   | Multi-client support          |
|   📅   | Cross-platform support        |
|   📅   | Official SDKs                 |
|   📅   | Import / Export               |
|   📅   | Comprehensive documentation   |

### **Status Legend**
* 🚧 In Development
* ✅ Implemented
* 🧪 Experimental
* 📅 Planned
* ❌ Removed / Unsupported

## Getting Started
### Requirements
* Windows
* No external database server required

### Installation
Download the latest PebbleDB installer from the GitHub Releases page.

Run the installer and follow the setup wizard to choose the installation location.

### CLI
After installation, launch PebbleDB from the Start Menu or run the executable directly.

Use the `HELP` command to view the available commands

### Built-in Server
PebbleDB includes a built-in HTTP server for applications that need to interact with the database through an API.

Start the server using:

```PebbleDB > SERVER START```

Check its status: 

```PebbleDB > SERVER STATUS```

Stop the server: 

```PebbleDB > SERVER STOP```

### HTTP API
The built-in server provides a simple RESTful API for interacting with the database.

> **NOTE:** The server currently runs on `localhost` only. Network access and multi-client support are
> not yet available because authentication and other server security features have not been implemented.
> These capabilities are planned for future releases.

#### Database Endpoints
|  Method  | Endpoint                 | Description           |
|:--------:|:-------------------------|:----------------------|
|  `GET`   | `/databases`             | List all databases    |
|  `POST`  | `/databases`             | Create a new database |
| `DELETE` | `/databases/{database}`  | Delete a database     |

#### Table Endpoints
|  Method  | Endpoint                               | Description                   |
|:--------:|:---------------------------------------|:------------------------------|
|  `GET`   | `/databases/{database}/tables`         | List all tables in a database |
|  `POST`  | `/databases/{database}/tables`         | Create a new table            |
| `DELETE` | `/databases/{database}/tables/{table}` | Drop a table                  |
|  `GET`   | `/databases/{database}/tables/{table}` | Get table information         |

#### Record Endpoints
| Method | Endpoint                                            | Description      |
|:------:|:----------------------------------------------------|:-----------------|
| `POST` | `/databases/{database}/tables/{table}/records`      | Add a new record |
| `GET`  | `/databases/{database}/tables/{table}/records/{id}` | Get a record     |
| `PUT`  | `/databases/{database}/tables/{table}/records/{id}` | Update a record  |

### Request Body
Certain endpoints require a request body to be sent with the request.

#### Database
##### Create a database
```json
{
  "db_name": "<database>"
}
```

#### Table
##### Create a table
```json
{
  "table_name": "<table name>",
  "columns": [
    ["Column Name", "Data Type"],
    ["Column Name", "Data Type"],
    ["Column Name", "Data Type"]
  ]
}
```

#### Record
##### Insert a record
```json
{
  "data": [
    "value",
    "value",
    "value"
  ]
}
```
##### Update a record
```json
{
  "updated_data": [
    "value",
    "value",
    "value"
  ]
}
```

### Data Types
PebbleDB currently supports the following data types:

When defining a column, the data type must use the exact keyword shown below.

**NOTE: Data type keywords are case-sensitive.**

| Data Type | Keyword   | Example                    |
|:---------:|:----------|:---------------------------|
|  Integer  | `Integer` | `["ID", "Integer"]`        |
|   Float   | `Float`   | `["Temperature", "Float"]` |
|   Text    | `Text`    | `["NAME", "Text"]`         |
|  Boolean  | `Boolean` | `["Status", "Boolean"]`    |

For example, a table can be created with the following column definitions:
```json
{
  "table_name": "employees",
  "columns": [
    ["Employee ID", "Integer"],
    ["Name", "Text"],
    ["Performance", "Float"],
    ["Status", "Boolean"]
  ]
}
```

### Data Storage
PebbleDB stores databases locally using its own binary `.peb` file format.

Application data is stored in the user's local application data directory
```%LOCALAPPDATA%\PebbleDB\data```

## Roadmap
### Version 0.1
* Interactive CLI
* Database management
* Table management
* Record CRUD operations
* Persistent storage
* Built-in local HTTP server
* Basic CLI help
* Initial documentation

### Version 0.2
* Filtering
* Sorting
* Constraints
* Additional data types
* Import / Export

### Version 0.3
* Transactions
* Indexes
* Performance improvements
* Improved parser

### Version 0.4
* Authentication
* Multi-client support
* Server improvements

### Version 1.0
* Stable API
* SDKs
* Documentation
* Testing
* Benchmarks
* Cross-platform support

## Project Status
PebbleDB **v0.1.0** is the first usable release of the project.

The release provides database, table, and record management through the CLI, persistent `.peb` storage,
and a built-in HTTP server with REST API support.

Future releases will expand PebbleDB's database capabilities, improve its storage and query systems, and broaden
application integration.

## License
See the repository for license information.