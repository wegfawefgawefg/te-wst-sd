# te-wst-sd
te-wst-sd

# What is this?
This is a design draft for a system given what I believe to be the current constraints.
Not being in a position to ask for clarification, I have made some assumptions. 
Given even a short time to ask for clarification, I would have asked for more details on the expected data volume, frequency, sizes, use, etc, and the whole system could be totally different. 

## The Big Assumptions
- small data may be submitted by machines, regularly.
- large data is being submitted, irregularly. (maybe by humans via interface)
- deployment architecture choice is not a strict requirement, as long as the requirements are met.

# Overview
There is a server that is managing requests to a database. 
Ive chosen to ignore the CSV requirement, and go with a sql style database.
Normal database backups and versioning gives data fault tolerance.
Small submissions are handled by the server, and large submissions handled in parts via streaming.
All request success and rejection states will be clearly defined, and strictly adhered to.

## Excalidraw System Diagram
https://excalidraw.com/#json=Gj5jvj2byTdAJr0Ac4toD,sXYLfmFhpNFtwuiloqdh_w
- check excalidraw.svg

# Process 
## Initial Thoughts, and Required Clarification
- If I expected tons of tiny requests, I would go with a job queue and workers.
- How do I know which csv file to append to?
- Why am I storing these as physical csv files on bucket? What the #!@&?
- What kind of data volume are we expecting here?
- Serverless... Lets assume AWS Lambda? 
- 3 hours is very short for such important decisions.
- Fault tolerance and speed were emphasized a few times, unfortunately I cannot ask for clarification, but I will 
assume given the emphasis that its important.
- Speed limitations means for a non compiled language, we're looking at a job queue.
- The system should be designed to minimize costs. (How about development costs, lets avoid the microservices.)
    If we need to scale, we can make a job queue, and have workers later.
- I'm not really concerned if the server is aws lambda or not. It's not that important to me. 
If we dont need a job queue, we don't need lots of microservices.

### Traps
- loading the csvs fully into memory
- bucket writes may be immutable
- csv's 
- amazon bucket
- assuming the data is so large a single db cant handle it


### Some Failure modes
- malformed csv
- malformed file
- scary file
- incomplete requests
- giant input csvs
- too many requests
- file too big

## Realistic Plan
- Store the transactions in a database, and then have a server/awslambda that reads the csv and appends to the correct table. This allows for metadata on the added data, and gives some fault tolerance for undoing, etc.
- Seperate submissions into small and big, if big ones are expected. (GIS data?)
- Provide specific failure case enums, for whoever is writing the ui/backend. (Parseable into arbitrary language)

### Development Steps
1. finish clarifying requirements
2. make a faux user request server
3. write db api
4. get db up (local)
5. test db/api
6. finish small file routes

7. test small file routes

7. big file routes
8. big file parts routes

9. test big file routes

10. rate test the server
11. try to kill the server / db

12. research database versioning and backup options
13. try them

14. deployment research
15. deployment
16. test deployment

review etc

