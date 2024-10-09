# TODO

## Version 1.0
- [x] Fix startup screen
- [x] Unify experience for add account and edit account
- [x] Fix bug where changed master password renders program unusable due to missing to update the dbs key alongside
- [ ] Control line for each state
  - [x] Create terminal context function for footer
  - [x] Main menu state
  - [x] Enter master password state
  - [x] Set master password state
  - [x] Add Account state
  - [x] Edit Account state
  - [x] List Accounts state
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
- [ ] Password handling
  - [ ] Hide passwords correctly
  - [ ] Handle cursor gracefully
  - [ ] Add option to show password on editing

## Version 1.1
- [ ] Select database storage location
  - [ ] Allow to select the database location when none is found
  - [ ] Scan all drives attached for a database
  - [ ] List databases if multiple are found
- [ ] Error state
  - [ ] Add fallback state to jump into if an error occurs
  - [ ] Remove unwrap and expect statements and redirect them to the fallback state