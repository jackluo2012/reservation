# Core Reservation Service

- Feature Name: core-reservation-service
- Start Date: 2022-10-08 09:33:54

## Summary

A core reservation service that solves the problem of reserving a resource for a period of time. We leverage postgres EXCLUDE constraints to ensure that only one reservation can be made for a given resource at a given time.

## Motivation

We need a common solution for various reservation requirements: 1) calendar booking; 2) hotel/room booking; 3) meeting room booking; 4) parking lot booking; 5) etc. Repeatedly building features for these requirements is a waste of time and resources. We should have a common solution that can be used by all teams.

## Guide-level explanation

Basic architecture:

![basic arch](images/arch1.jpg)

### Service interface

We would use gRPC as a service interface. Below is the proto definition:
```proto
message Reservation {
    string resource_id = 1;
}

service ReservationService {}
```
