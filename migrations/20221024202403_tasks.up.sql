-- Add up migration script here
CREATE TABLE task_view
(
    view_id TEXT                        NOT NULL,
    version BIGINT CHECK (version >= 0) NOT NULL,
    payload JSON                        NOT NULL,
    PRIMARY KEY (view_id)
)
