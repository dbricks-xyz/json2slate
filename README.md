Automatic [slate docs](https://github.com/slatedocs/slate) generator from files supplied into the "in" folder. Typically those files are generated as side artifacts when tests are ran, and pasted in.

This ensures that API examples:
- are always up to date with latest tests
- never have typos / mismatches
- are generated programmatically, with no need for manual input

How it works:
1. Run the tests in `dbricks-server`
2. Paste the artifacts into `in`
3. Run the rust binary
4. Take the generated docs from `out` and paste into `source/includes/out` in `dbricks-docs`
5. 🚀