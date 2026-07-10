# PebbleDB
> A lightweight, CLI-first local database engine for rapid prototyping and application development.

## About
PebbleDB is an open-source, lightweight database engine designed for developers who need a fast and portable solution
for local application development. The project aims to simplify database setup by providing a single executable that
can create, manage, and server local databases without requiring traditional database servers or containerization
tools.

Built around a modular architecture, PebbleDB consists of a command-line interface, parser, database engine, storage
engine, and an optional built-in server. While development begins with a CLI-first experience, the long-term vision
is to support multiple interfaces for applications to interact with the database.

PebbleDB is also developed as an educational project, documenting the design and implementation of a database system
from the groud up while remaining useful for rapid prototyping and local development.

## Why PebbleDB?
Modern application development often requires installing and configuring a database before writing application code.
While production database systems such as MySQL and PostgreSQL are excellent for real-world deployments, they can
introduce uneccesarry setup during prototyping, hackathons, classroom activities, and personal projects.

PebbleDB reduces this friction by providing a lightweight, local-first database that starts quickly, requires minimal
configuration, and is easy to integrate into development workflows.

## Features
The following table outlines PebbleDB's planned capabilities and their current development status. As the project 
evolves, features will progress from planned to implemented throughout each release.

| Status | Feature                       |
|:------:|-------------------------------|
|   🚧   | Interactive CLI               |
|   🚧   | Database creation             |
|   🚧   | Table creation and management |
|   🚧   | CRUD operations               |
|   🚧   | Persistent local storage      |
|   📅   | Filtering and sorting         |
|   📅   | Constraints                   |
|   📅   | Transactions                  |
|   📅   | Indexing                      |
|   📅   | Built-in local server         |
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
* ❌ 

## Current Status
PebbleDB is currently in active development.

The repository currently contains the project's software design and architecture documentation. Implementation begins
with Version 0.1, starting with the core database engine, command-line interface, and persistent storage.

As the project evolves, both the implementation and documentation will continue to improve.

## Roadmap
### Version 0.1
* Interactive CLI
* Database creation
* Table creation
* CRUD operations
* Persistent storage
* Documentation

### Version 0.2
* Filtering
* Sorting
* Constraints
* Data Types
* Import / Export

### Version 0.3
* Transactions
* Indexes
* Performance improvements
* Better parser

### Version 0.4
* Built-in server
* Authentication
* Multi-client support

### Version 1.0
* Stable API
* SDKs
* Documentation
* Testing
* Benchmarks
