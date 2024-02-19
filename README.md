# te-wst-sd
te-wst-sd

# Excalidraw System Diagram
https://excalidraw.com/#json=Gj5jvj2byTdAJr0Ac4toD,sXYLfmFhpNFtwuiloqdh_w
- check excalidraw.svg

## Initial Thoughts, and Required Clarification
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

### traps
- loading the csvs fully into memory
- bucket writes may be immutable
- csv's 
- amazon bucket


### failure modes
- malformed csv
- malformed file
- scary file
- incomplete requests
- giant input csvs
- too many requests
- file too big

## Realistic Plan
- Store the transactions in a database, and then have a lambda function that reads the csv and appends to the correct table. This allows for metadata on the added data, and gives some fault tolerance for undoing, etc.
- Provide specific failure case enums, for whoever is writing the ui/backend. (Parseable into arbitrary language)

### steps
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

