# randevu (prototype)
Universal daily deterministic probabilistic pseudorandom offline reminders for anything

https://github.com/TypicalHog/randevu

Open "RANDEVU.randevu" in a text editor to add/remove/edit items,
if it doesn't exist, it will be created on the first run of the program.
Each line represents one item and should be in the format: "ID N\n"
Empty lines, IDs with spaces and any other invalid values will be ignored.

ID is a string (uppercase A-Z, digits 0-9, no spaces or any other characters).
Other characters should only be used for case sensitive identifiers,
or ones that contain symbols and characters like dashes - YOUTUBE links etc.

N is an integer starting at 0. It represents the desired randevu level.
A value of 5 would mean the user would like to get reminders for ID,
on average, every 32 (2^5) days.
2^N = average number of days between randevus of level N for a certain item
1 / 2^N = probability a randevu of level N will occur for a certain item on any given day
N 0 is always active.

Run the program each day to see which items are active on that day.
Even though the program runs offline, all users will get the same outputs
for the same items on the same days as long as they enter the same ID and N.
Every reminder with level N also always lands on the same days as any lower level
reminders. For example: randevu 4 is one the same days as 3, 2, 1, 0.

Randevus are in the format: "ID M/N" where M is the highest randevu for a certain items
on any given day and N is a user set number loaded from the file
Randevus will be printed for any item where M >= N.
