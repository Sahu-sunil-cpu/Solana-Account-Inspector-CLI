##Solana Account Inspector CLI
---
#Features
---
```
inspect <Cluster> <Publickey> 
```
1. fetch the details using solana rpc url
2. parse the data into needed one
3. suppoer different types of account fetching i.e. mint account, data account, and program account
4. output on cmd is easily readable
5. unit tests for edge cases and each feature
6. showing relevant errors to the end user
7. errors can be wrong public key, any other argument instead of available cluster and fetching error
8. supporing the help argument