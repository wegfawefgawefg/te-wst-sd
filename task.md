Tensor Energy | Work sample test | System design

### **Overview**

In this work sample test, you will be asked to design a system that can process CSV files uploaded by users. The contents of the user uploaded CSV files should be appended to one of 3 existing CSV files in an existing S3 bucket. Before opening and reading the CSV file uploaded by users, we do not know which of the 3 existing CSV files the contents should be merged into. The system should be able to handle a high volume of requests, should be highly available and fault-tolerant.

### **Requirements**

Your solution should meet the following requirements:

- The system should be designed using a serverless architecture.
- The system should be able to handle a high volume of requests.
- The system should be able to process each request in real-time.
- The system should be highly available and fault-tolerant.
- The system should be able to scale automatically based on the incoming traffic.
- The system should be designed to minimize costs.
- Users should be able to upload CSV files to the system.
- The contents of the CSV files should be appended to one of 3 existing CSV file in an existing S3 bucket.
- You should use an IaaS tool like AWS CDK or Terraform to deploy the system.
- You should push the code to a GitHub repository that you share with us.

### **Deliverables**

You should provide the following deliverables:

- A high-level architecture diagram that shows the components of the system and how they interact with each other.
- A detailed description of each component of the system, including the technologies used and how they are configured.
- A list of assumptions and trade-offs made during the design process.
- A list of potential issues and how they can be mitigated.
- The code for deploying the system using an IaaS tool like AWS CDK or Terraform.
- A README file that explains how to deploy and test the system.

### **Timeframe**

Please complete the work sample test in not more than 3 hours.

### **Submission**

Please send your solution as a zip file in reply to the email you’ve received with a README file that explains your solution and how to deploy and test it.