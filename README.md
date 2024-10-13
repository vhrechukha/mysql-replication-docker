# MySQL Master-Slave Replication using Docker

This project demonstrates MySQL master-slave replication using Docker with one master (`mysql-m`) and two slaves (`mysql-s1`, `mysql-s2`). We tested replication by inserting data on the master, verifying it on the slaves, and checking replication when a slave was stopped. Additionally, we tested the impact of altering the schema on a slave.


## Logs Showing Successful Replication Between Master and Slaves

1. Master Status (mysql-m)
```
docker exec -it mysql-m mysql -uroot -prootpassword -e 'SHOW MASTER STATUS\G'

*************************** 1. row ***************************
             File: mysql-bin.000003
         Position: 157
     Binlog_Do_DB: testdb
 Binlog_Ignore_DB: 
Executed_Gtid_Set:
```
This log confirms the master is generating binlog events from file mysql-bin.000003 at position 157.

2. Slave Status Check (mysql-s1)
```
SHOW SLAVE STATUS\G;
```
Output from mysql-s1:

```
*************************** 1. row ***************************
               Slave_IO_State: Waiting for source to send event
                  Master_Host: mysql-m
                  Master_User: replica
                  Master_Port: 3306
                Connect_Retry: 60
              Master_Log_File: mysql-bin.000003
          Read_Master_Log_Pos: 1328
               Relay_Log_File: slave-relay-bin.000002
                Relay_Log_Pos: 1497
        Relay_Master_Log_File: mysql-bin.000003
             Slave_IO_Running: Yes
            Slave_SQL_Running: Yes
              Seconds_Behind_Master: 0
```
This shows that replication is working correctly on mysql-s1, with both Slave_IO_Running and Slave_SQL_Running being Yes, and Seconds_Behind_Master at 0.

3. Slave Status Check (mysql-s2)
Output from mysql-s2:

```
*************************** 1. row ***************************
               Slave_IO_State: Waiting for source to send event
                  Master_Host: mysql-m
                  Master_User: replica
                  Master_Port: 3306
              Master_Log_File: mysql-bin.000003
          Read_Master_Log_Pos: 1328
               Relay_Log_File: slave-relay-bin.000002
                Relay_Log_Pos: 1497
             Slave_IO_Running: Yes
            Slave_SQL_Running: Yes
              Seconds_Behind_Master: 0
```
Similarly, mysql-s2 is fully synchronized with the master, and replication is functioning correctly.

4. Data Insertion on the Master (mysql-m)
We inserted test data into the master database:

```
docker exec -it mysql-m mysql -uroot -prootpassword -e "INSERT INTO testdb.test_table (name) VALUES ('Replication Test 1'), ('Replication Test 2');"
```
5. Verifying Data Replication on the Slaves
Data on mysql-s1:
```
docker exec -it mysql-s1 mysql -uroot -prootpassword -e "SELECT * FROM testdb.test_table;"
```
Output:

```
+----+-------------------+
| id | name              |
+----+-------------------+
|  1 | Replication Test 1 |
|  2 | Replication Test 2 |
+----+-------------------+
```
Data on mysql-s2:
```
docker exec -it mysql-s2 mysql -uroot -prootpassword -e "SELECT * FROM testdb.test_table;"
```
Output:

```
+----+-------------------+
| id | name              |
+----+-------------------+
|  1 | Replication Test 1 |
|  2 | Replication Test 2 |
+----+-------------------+
```
Both slaves show the same data, indicating that the replication is working properly.

6. Stopping a Slave (mysql-s1)
We simulated a failure by stopping the first slave (mysql-s1):

```
docker stop mysql-s1
```
7. Verifying Data Replication on mysql-s2 After Stopping mysql-s1
New data was inserted into the master:

```
docker exec -it mysql-m mysql -uroot -prootpassword -e "INSERT INTO testdb.test_table (name) VALUES ('Replication Test 3');"
```
Checking the data on the remaining slave (mysql-s2):

```
docker exec -it mysql-s2 mysql -uroot -prootpassword -e "SELECT * FROM testdb.test_table;"
```
Output:

```
+----+-------------------+
| id | name              |
+----+-------------------+
|  1 | Replication Test 1 |
|  2 | Replication Test 2 |
|  3 | Replication Test 3 |
+----+-------------------+
```
This confirms that mysql-s2 continues to replicate data from the master even after mysql-s1 was stopped.
