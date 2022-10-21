-- Add up migration script here
-- a single table is used for all events in the cqrs system
CREATE TABLE events
(
    aggregate_type TEXT                         NOT NULL,
    aggregate_id   TEXT                         NOT NULL,
    sequence       BIGINT CHECK (sequence >= 0) NOT NULL,
    event_type     TEXT                         NOT NULL,
    event_version  TEXT                         NOT NULL,
    payload        JSON                         NOT NULL,
    metadata       JSON                         NOT NULL,
    PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);

-- this table is only needed if snapshotting is employed
CREATE TABLE snapshots
(
    aggregate_type   TEXT                                 NOT NULL,
    aggregate_id     TEXT                                 NOT NULL,
    last_sequence    BIGINT CHECK (last_sequence >= 0)    NOT NULL,
    current_snapshot BIGINT CHECK (current_snapshot >= 0) NOT NULL,
    payload          JSON                                 NOT NULL,
    PRIMARY KEY (aggregate_type, aggregate_id, last_sequence)
);