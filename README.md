# randevu (prototype)
Universal Daily Probabilistic Offline Reminders for Anything

https://github.com/TypicalHog/randevu

- OFFLINE - no internet connection required
- FOSS - in public domain
- DAILY - reminders are generated each UTC date
- DETERMINISTIC - reproducible for any object on any date
- PSEUDORANDOM - reminders are spaced randomly and uniformly
- PROBABILISTIC - reminders are calculated using the probabilistic algorithm
- UNIVERSAL - reminders for the same object are the same for everyone
- GENERIC - applicable to anything and everything
- ADJUSTABLE - frequency of reminders for each object can be adjusted

Open "RANDEVU.rdv" in a text editor to add/remove/edit items,
if it doesn't exist, it will be created on the first run of the program.
Each line represents one item and should be in the format: "ID N\n"
Empty lines, IDs with spaces and any other invalid values will be ignored.

ID is a string (preferably uppercase A-Z, 0-9). No spaces allowed.
Spaces and any other characters should be replaced with a single underscore "_".
Characters outside of this set should only be used for case sensitive external identifiers,
or ones that contain other symbols and characters, like YOUTUBE links.

EXAMPLES:
>XONOTIC (only uppercase)
>
>THE_MATRIX_1999 (movies should have a year at the end)
>
>GRAND_THEFT_AUTO_5 (full name, roman numerals should be replaced with arabic ones)
>
>ASAP_ROCKY ($ should be replaced with S, same for other similar instances)
>
>NO_MANS_SKY (apostrophe in MAN'S should be dropped)
>
>HARRY_POTTER_SMOKES_WEED_Cdfkq2Nmb3c (video ID should be appended to the video title, double underscore is fine if the ID starts with one)

Why tho? you may ask.
**To make sure we all get the same reminders for the same objects.**
Even just a single character that's different causes the system to generate completely different reminders.
You could append certain items with some extra characters to get completely different and unrelated reminders from other people.
WEED, Weed, WEED1, WEED_ and WEED420 are all treated as different objects.
Think of the object IDs like passwords. Same password gets you the same reminders as other people.

N is an integer 0 and up. It represents the desired randevu level.
N = 5 would mean the user would get reminders for a specific item every 32 (2^5) days, on average.
2^N = average number of days between randevus of level N for an item (can vary)
1 / 2^N = probability a randevu of level N will occur for an item on any given day
N = 0 is always active while any other level N is probabilistic.

Run the program each day to see which items have a randevu that day.
Even though the program runs offline, all users will get the same outputs
for the same items on the same days as long as they enter the same ID and N.
Every reminder with level N also always lands on the same days as any lower level
reminders. For example: randevu 4 is on the same days as 3, 2, 1, 0. (for the same ID)

Randevus are shown to the user in the format:
"ID M/N" where M is the highest randevu for a certain item
on a given day and N is a user set number loaded from the file
Randevu for an item is only shown for items where M >= N.

Proposal:
https://www.reddit.com/r/Lightbulb/comments/14eqqa8/universal_daily_deterministic_pseudorandom

Discussion:
https://www.reddit.com/r/randevu
