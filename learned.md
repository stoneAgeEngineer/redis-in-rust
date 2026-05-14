- REDIS uses RESP (REDIS SERIALIZATION PROTOCAL) protocol for client-server communication
- \r\n . \r stands for carriage return . This will return the cursor to the starting point of current line . By adding \n it moves to next line . If \n not added it will overwrite the existing content

# Auth
- ACL (Access Control List) WHOAMI return the username associated with the current connection . By default every new connection is authenticated with "default" username 
- flags in redis. flag is a set of attributes that describe how a user behaves or what special permissions they have Each flag is a short label that defines part of the user’s configuration.
- Auth allows to set password for user and if password is set then later clients connected also have to auth first with password 

# LIST
- Redis RPUSH stores linked list data in binary string format and what ever you send it stores in binary string format

