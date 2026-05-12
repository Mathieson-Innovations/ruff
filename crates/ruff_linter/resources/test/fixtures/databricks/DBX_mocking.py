from unittest.mock import MagicMock, patch, create_autospec

# DBX019
mock = MagicMock()

# DBX018
patch("databricks.sdk.WorkspaceClient")

# DBX021
MagicMock()
create_autospec(dict)
patch("os.path.join")
