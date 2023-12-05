# Challenges

Here are the instructions to solve each challenge

## Challenge 01

#### The Challenge

A spy is sending encrypted messages.

Your mission is to create a program that decodes the messages.

The messages are words separated by spaces like this: cat dog dog car Cat doG
sun

We need the program to return the number of times each word appears in the
message, regardless of whether it is in uppercase or lowercase.

The result will be a text string with the word and the number of times it
appears in the message, in this format: cat2dog3car1sun1

The words are sorted by their first appearance in the message!

#### More Examples

house housess -> houses1house1housess1

#### How to Solve It

1. Solve the message you will find in this file: /data/message_01.txt

## Challenge 02

### Mini Compiler Challenge

In the world of espionage, a coding language is used with symbols that perform
simple mathematical operations.

Your task is to create a mini compiler that interprets and executes programs
written in this symbol language.

The operations of the language are as follows:

"#" Increases the numeric value by 1. "@" Decreases the numeric value by 1. "\*"
Multiplies the numeric value by itself. "&" Prints the current numeric value.
The initial numeric value is 0 and the operations should be applied in the order
in which they appear in the string of symbols.

### Input Examples:

Input: "##\*&" Expected Output: "4" Explanation: Increment (1), increment (2),
multiply (4), print (4).

Input: "&##&\*&@&" Expected Output: "0243" Explanation: Print (0), increment
(1), increment (2), print (2), multiply (4), print (4), decrement (3), print
(3).

### Your Challenge:

Develop a mini compiler that takes a text string and returns another text string
with the result of executing the program.

### How to Solve It

1. Solve the message you will find in this file: /data/message_02.txt

## Challenge 3

### The Spy Encryption Challenge

A group of spies has discovered that their message encryption system is
compromised.

They have found some passwords that do not comply with theEncryption Security
Policy that was established when they were created.

To solve the problem, they have created a list (your entry to the challenge) of
passwords (according to the corrupted database) and the security policy when
that key was established.

Example of the list:

2-4 f: fgff 4-6 z: zzzsg 1-6 h: hhhhhh Each line indicates, separated by :, the
key policy and the key itself.

The key policy specifies the minimum and maximum number of times a given
character must appear for the key to be valid. For example, 2-4 f means that the
key must contain f at least 2 times and at most 4 times.

Knowing this, in the previous example, there are 2 valid keys but one is not:

The second key, zzzsg, is not valid; it contains the letter z 3 times, but needs
at least 4. The first and third keys are valid: they contain the appropriate
amount of f and h, respectively, according to their policies.

### Your challenge:

Determine how many encryption keys are valid according to their policies.

### How to solve it

1. Analyze the list of policies and encryption keys that you will find in this
   file: /data/encryption_policies.txt

2. Create a program that returns the 42nd invalid key (of all the invalid keys,
   the 42nd in order of appearance). For example: submit bqamidgewtbuz

## Challenge 4

### Hackers Damage File System

In a world where information is power, a hacker known as Savipo Yatar discovers
a series of files on a highly protected server.

These files contain secrets that could change the course of history. But there's
a problem: some of the files are fake, designed to deceive intruders. Savipo
Yatar must quickly determine which files are real and which are fake.

Each file has a name with two parts, separated by a hyphen (-). The first part
is an alphanumeric string, and the second is achecksum, which is a string formed
by the characters that only appear once in the first part and in the order in
which they appear.

Write a program that determines whether a file is real or fake based on these
rules.

Examples:

File name: xyzz33-xy Result: ✅ Real (The checksum is valid) File name:
abcca1-ab1 Result: ❌ Fake (The checksum should be b1, it's incorrect) File
name: abbc11-ca Result: ❌ Fake (The checksum should be ac, the order is
incorrect) Each line indicates the file name and its corresponding checksum,
separated by a hyphen (-).

### How to Solve It

1. Analyze the list of file names and their checksums that you will find in this
   file: /data/files_quarantine.txt

2. Look for the real file number 33 (of all the real files, the 33rd in order of
   appearance) and submit its checksum with submit. For example, if the file is
   xyzz33-xy, you would do: submit xy

## Challenge 05

### The Final Problem

Finally, hackers have managed to access the database and have corrupted it. But
it seems they have left a hidden message in the database. Can you find it?

Our database is in .csv format. The columns areid,username,email,age,location.

A user is only valid if:

- id: exists and is alphanumeric
- username: exists and is alphanumeric
- email: exists and is valid (follows the pattern user@domain.com)
- age: is optional but if it appears it is a number
- location: is optional but if it appears it is a text string Examples:

Entry: 1a421fa,alex,alex9@gmail.com,18,Barcelona Result: ✅ Valid

Entry: 9412p*m,maria,mb@hotmail.com,22,CDMX Result: ❌ Invalid (id is not
alphanumeric, the * is extra)

Entry: 494ee0,madeval,mdv@twitch.tv,, Result: ✅ Valid (age and location are
optional) Entry: 494ee0,madeval,twitch.tv,22,Montevideo Result: ❌ Invalid
(email is not valid)

### How to Solve It

1. Analyze the list of database entries and detect the invalid
   ones: /data/database_attacked.txt

2. Find the first letter of the username of all invalid users. Gather them in
   the order of appearance and uncover the hidden message.
