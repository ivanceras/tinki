
## Diwata is comprised with 3 major components

1. Rustorm
2. Intel
3. Client

```bob

             .----------.            .- - - - - - - - - -.
            (  Rustorm   )  <|- - - -! Database metadata !
             `----------'            `- - - - - - - - - -'
                   |    
                   |  
                   |   
                   |  
                   v
               _________
              /         \            . - - - - - - - - - - .
             /   Intel   \  <|- - - -! Data interpretation !
             \           /           `- - - - - - - - - - -'
              \_________/
                   |          
                   |
                   | 
                   | 
                   v
        +--------------------+    
       /                      \
      /        Client          \
     /                          \
    /____________________________\

```

#### Rustorm 
Rustorm is the database ORM that takes care of extract table meta data
from the underlying database.

#### Intel 
Intel is the intellisense of the system which does inference of an interpreting
the data being instrospected. This contains the logic for determining the presentation
of the data to the client. It fills the gap when there is not enough information
extracted from the system.

#### Client 
The client does pull the curtain API and display the content in a nice
and structure presentation. Aside from just being a interactice Rich client application,
the client is also responsible for mapping the URL to their corresponding application
state and modules activated. Sharing the URL to other users with the same login
credentials will show the same exact content.

[Back to Diwata](../Diwata.md)
