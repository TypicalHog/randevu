# randevu (prototype)
Universal Probabilistic Daily Reminders for Anything

https://github.com/TypicalHog/randevu

- OFFLINE - no internet connection required
- FOSS - unlicensed, in public domain
- DAILY - reminders are generated on a daily basis (UTC)
- DETERMINISTIC - easily reproducible for any object and any date
- PSEUDORANDOM - reminders are spaced randomly and uniformly
- PROBABILISTIC - reminders are calculated using the probabilistic algorithm
- UNIVERSAL - reminders for the same object are the same for everyone
- GENERIC - applicable to anything and everything, literally
- ADJUSTABLE - frequency of reminders for each object can be adjusted

---

## How to use

Open "RANDEVU.rdv" in a text editor to add/remove/edit items,
if it doesn't exist, it will be created on the first run of the program.
Each line represents one item and should be in the format: "ID N\n"
Empty lines, IDs with spaces and any other invalid values will be ignored.

ID is a string (preferably uppercase A-Z, 0-9). No spaces allowed.
Spaces and any other characters should be replaced with a single underscore "_".
Characters outside of this set should only be used for case sensitive external identifiers,
or ones that contain other symbols and characters, like YOUTUBE links.

NAMING CONVENTION EXAMPLES:
>**XONOTIC** (all letters should be uppercase)
>
>**THE_MATRIX_1999** (movies should have a year of release at the end)
>
>**GRAND_THEFT_AUTO_5** (objects should be referenced by their full name, roman numerals should be replaced with arabic ones)
>
>**ASAP_ROCKY** ($ should be replaced with S, same for other similar instances)
>
>**NOTEPAD_PLUS_PLUS** (++ should be replaced with _PLUS_PLUS, same for C++)
>
>**C_SHARP** (# should be replaced with _SHARP)
>
>**YEAR_2000** (years should have a YEAR_ prefix)
>
>**2023-08-25** (dates should be ISO 8601, YYYY-MM-DD)
>
>**NO_MANS_SKY** (apostrophe in MAN'S should be dropped)
>
>**GETTING_BANGED_BY_GREEN_BOOMERS_MINECRAFT_BETA_1_7_3_SOLO_SURVIVAL_NO_COMMENTARY_OJzsmWBQE3I** (brackets, quotation marks, colons and other punctuation should be dropped, periods in version numbers like 1.7.3 should be replaced with underscores)
>
>**HARRY_POTTER_SMOKES_WEED_Cdfkq2Nmb3c** (video ID should be appended to the video title, double underscore is fine if the ID starts with one)

WHY THO? YOU MAY ASK.

**TO MAKE SURE WE ALL GET THE SAME REMINDERS FOR THE SAME THINGS.**

Even just a single character that's different causes the system to generate completely different reminders.
You could append certain items with some extra characters to get completely different and unrelated reminders from other people, if you really wanted that.
WEED, Weed, WEED1, WEED_ and WEED420 are all treated as different objects.
Think of the object IDs like passwords. Same password gets you the same reminders as other people.
NOTE: Naming convention is not set in stone, at least not yet.

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

---

X:
>https://x.com/randevurdv

PROPOSAL:
>https://www.reddit.com/r/Lightbulb/comments/14eqqa8/universal_daily_deterministic_pseudorandom

DISCUSSION:
>https://www.reddit.com/r/randevu
