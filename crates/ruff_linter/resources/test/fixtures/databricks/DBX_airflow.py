# DBX001 & DBX002
DatabricksCreateJobsOperator(
    task_id="job",
    new_cluster={
        "spark_version": "10.4.x-scala2.12",
        "node_type_id": "i3.xlarge",
    }
)

# OK
DatabricksCreateJobsOperator(
    task_id="job2",
    new_cluster={
        "spark_version": "13.3.x-scala2.12",
        "data_security_mode": "SINGLE_USER",
    }
)
