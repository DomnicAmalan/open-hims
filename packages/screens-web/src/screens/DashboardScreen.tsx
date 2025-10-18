import React, { useMemo } from 'react';
import { Card, Grid, Group, Text, Button, Container } from '@mantine/core';
import { useSelector } from 'react-redux';
import { createSelector } from '@reduxjs/toolkit';
import type { RootState } from '@open-hims/store';

// Define proper types for the selector
interface DashboardData {
  patients: any[];
  loading: boolean;
  error: string | null;
}

// Memoized selector to prevent unnecessary re-renders
const selectDashboardData = createSelector(
  (state: RootState) => state.patients.patients,
  (state: RootState) => state.patients.loading.fetchPatients,
  (state: RootState) => state.patients.error,
  (patients: any[], loading: boolean, error: string | null): DashboardData => ({
    patients,
    loading,
    error,
  })
);

export function DashboardScreen() {
  const { patients, loading, error } = useSelector<RootState, DashboardData>(selectDashboardData);

  // Memoize computed values
  const stats = useMemo(() => ({
    totalPatients: patients.length,
    totalRecords: patients.length,
    status: error ? 'Error' : 'Ready',
    statusColor: error ? 'red' : 'green'
  }), [patients.length, error]);

  return (
    <Container size="lg" py="md">
      <Text size="xl" fw="bold" mb="md">
        Healthcare Dashboard
      </Text>
      
      <Grid>
        <Grid.Col span={{ base: 12, md: 4 }}>
          <Card shadow="sm" padding="lg" radius="md" withBorder>
            <Group justify="space-between" mb="xs">
              <Text fw={500}>Total Patients</Text>
            </Group>
            <Text size="xl" fw="bold" c="blue">
              {loading ? '...' : stats.totalPatients}
            </Text>
          </Card>
        </Grid.Col>
        
        <Grid.Col span={{ base: 12, md: 4 }}>
          <Card shadow="sm" padding="lg" radius="md" withBorder>
            <Group justify="space-between" mb="xs">
              <Text fw={500}>Total Records</Text>
            </Group>
            <Text size="xl" fw="bold" c="green">
              {loading ? '...' : stats.totalRecords}
            </Text>
          </Card>
        </Grid.Col>
        
        <Grid.Col span={{ base: 12, md: 4 }}>
          <Card shadow="sm" padding="lg" radius="md" withBorder>
            <Group justify="space-between" mb="xs">
              <Text fw={500}>Status</Text>
            </Group>
            <Text size="xl" fw="bold" c={stats.statusColor}>
              {stats.status}
            </Text>
          </Card>
        </Grid.Col>
      </Grid>
      
      <Group mt="xl">
        <Button variant="filled">Manage Patients</Button>
        <Button variant="outline">View Reports</Button>
        <Button variant="light">Settings</Button>
      </Group>
    </Container>
  );
}