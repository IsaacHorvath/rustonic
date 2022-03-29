## Rustonic

A simple web app for playing music from a server via the Subsonic API.

Front-end only for now.

### Design Notes

- The main component is basically just a container
- The artist tab component holds a HashMap of ids, artists
- The album tab component holds a HashMap of ids, albums
- The artist tab sends an album list to the artists via an agent called EventBus

### Todo:

[ ] Fix main list
[ ] Fully move to HashMaps and id numbers
[ ] Add play queue component
[ ] Implement subsonic API on frontend
[ ] Implement the actual player
[ ] Proper caching
[ ] Build a backend
