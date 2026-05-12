DatabricksCreateJobsOperator(
    task_id="job",
    new_cluster={
        "spark_version": "10.4.x-scala2.12",
        "data_security_mode": "SINGLE_USER",
    }
)
