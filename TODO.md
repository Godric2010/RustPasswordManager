# TODO

## Version 1.0
- [x] Fix startup screen
- [ ] Unify experience for add account and edit account
  - [ ] Edit area for add account
  - [ ] Fix edit line for edit account
- [ ] Control line for each state
  - [ ] Main menu state
  - [ ] Enter master password state
  - [ ] Set master password state
  - [ ] Add Account state
  - [ ] Edit Account state
  - [ ] List Accounts state
  - [ ] Wipe Database
- [ ] Add scroll capability to List Accounts state
- [ ] Add exit option for each state
  - [ ] List Accounts state
  - [ ] Add Account state
  - [ ] Set Master Password
- [ ] Window management 
  - [ ] Define minimum space for app
  - [ ] Define maximum space for app
  - [ ] Scale app window based on available space
  - [ ] Output safe error if window is too small
  - [ ] Store all texts to display in a resource file

## Version 1.1
- [ ] Select database storage location
  - [ ] Allow to select the database location when none is found
  - [ ] Scan all drives attached for a database
  - [ ] List databases if multiple are found
- [ ] Error state
  - [ ] Add fallback state to jump into if an error occurs
  - [ ] Remove unwrap and expect statements and redirect them to the fallback state