# randevu (prototype)
Universal daily deterministic probabilistic pseudorandom offline reminders for anything

Open RANDEVU.randevu in a text editor to add/remove/edit items.
File will be created on first run if it doesn't exist.
Each line represents one item and should be in the format: "ID N\n"
Empty and invalid lines will be ignored.

ID is a string (uppercase A-Z, digits 0-9, no spaces or any other characters).
Other characters should only be used for case sensitive identifiers,
or ones that contain other characters like dashes - YOUTUBE links etc.

N is an integer starting at 0. Desired reminder level.
2^N = average number of days between reminders of level N for a certain item
1 / 2^N = probability a reminder of level N will occur for a certain item on any given day

Run the program each day to see which items are active.

Reminders are in the format: "ID M/N" where M is the highest reminder for a certain items
on any given day and N is a user set number loaded from the file
Reminders will be printed for any item where M >= N.
