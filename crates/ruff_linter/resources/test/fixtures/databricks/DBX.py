# DBX003
dbutils.fs.cp("a", "b")

# DBX004
dbutils.fs.head("a")

# DBX005
dbutils.fs.ls("a")

# DBX006
dbutils.fs.mount("s3a://bucket", "/mnt/bucket")
dbutils.fs.unmount("/mnt/bucket")

# DBX007
dbutils.credentials.getSecret("scope", "key")

# DBX008
dbutils.notebook.run("notebook", 60)

# DBX009
token = "dapi1234567890abcdef1234567890abcdef"

# DBX010
dbutils.getDbutils()
spark.notebook().getContext().apiToken()

# DBX011
import databricks_cli

# DBX012
import boto3
s3 = "s3fs"

# DBX013 & DBX014
# Databricks notebook source
# MAGIC %run ./other
# COMMAND ----------
# cell 2

# DBX024
# MAGIC %pip install foo

# OK
# MAGIC %uv pip install foo

