# randevu (prototype)
Universal daily deterministic probabilistic pseudorandom offline reminders for anything

https://github.com/TypicalHog/randevu

Open "RANDEVU.randevu" in a text editor to add/remove/edit items,
if it doesn't exist, it will be created on the first run of the program.
Each line represents one item and should be in the format: "ID N\n"
Empty lines, IDs with spaces and any other invalid values will be ignored.

ID is a string (uppercase A-Z, 0-9). No spaces or any other characters.
Characters outside of this set should only be used for case sensitive external identifiers,
or ones that contain other symbols and characters, like YOUTUBE links.

N is an integer 0 and up. It represents the desired randevu level.
N = 5 would mean the user would get reminders for a specific item every 32 (2^5) days, on average.
2^N = average number of days between randevus of level N for an item
1 / 2^N = probability a randevu of level N will occur for an item on any given day
N = 0 is always active while any other level is probabilistic.

Run the program each day to see which items have a randevu that day.
Even though the program runs offline, all users will get the same outputs
for the same items on the same days as long as they enter the same ID and N.
Every reminder with level N also always lands on the same days as any lower level
reminders. For example: randevu 4 is on the same days as 3, 2, 1, 0. (for the same ID)

Randevus are shown to the user in the format:
"ID M/N" where M is the highest randevu for a certain item
on a given day and N is a user set number loaded from the file
Randevu for an item is only shown for items where M >= N.

Proposal and discussion:
https://www.reddit.com/r/Lightbulb/comments/14eqqa8/universal_daily_deterministic_pseudorandom/
