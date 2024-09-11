This is a Rust tool designed to import message data from various messaging platforms into a Postgres database.

## Features:
- Parses JSON files containing message data.
- Supports importing participants, messages, and reactions (implementation may vary depending on the platform).
- Connects to a Postgres database using a connection pool.
- Inserts data in batches for efficiency.
- Handles merging data from multiple import files (for the same conversation).

## Installation:
Clone this repository.
Run cargo build --release to build the tool.
Ensure you have a Postgres database set up with the appropriate schema (tables for participants, messages, and reactions).

Usage:
Bash
```bash
message-import --app <app_name> --destination postgres://user:password@host:port/database --input /path/to/folder/with/conversation/files --chunk-size 1000
```

Arguments:
- app: Name of the messaging app for which the import is intended (e.g., messenger, telegram, discord).
- destination: URL of the Postgres database holding the schema (e.g., postgres://user:password@host:port/database).
- max-connections: Maximum number of connections in the connection pool (default: 1).
- chunk-size: Size of each batch for inserting messages (default: very large, effectively all at once).
- input: Folder containing the JSON files with conversation data.

Integrating a new Messaging App:

The core logic for handling message data resides in the adapter module. To integrate a new messaging app, you'll need to implement the following traits for your specific conversation model:
- ConversationConverter: This trait defines a method to convert your conversation model (e.g., WhatsAppConversation) into the generic Conversation struct used for database interaction.
- ConversationLoader: This trait defines methods for loading participants, messages, and reactions from your conversation model and inserting them into the database.

Further Notes:
- Feel free to adjust the chunk_size argument based on your database performance and message volume.
- This is a basic example, and you might want to extend it with features like error handling, logging, and progress reporting.
