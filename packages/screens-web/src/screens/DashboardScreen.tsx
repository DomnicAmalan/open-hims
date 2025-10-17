import React from 'react';
import { Card, Grid, Group, Text, Button, Container } from '@mantine/core';
import { useSelector } from 'react-redux';
import type { RootState } from '@open-hims/store';

export function DashboardScreen() {
  const { patients, loading, error } = useSelector((state: RootState) => ({
    patients: state.patients.patients,
    loading: state.patients.loading.fetchPatients,
    error: state.patients.error,
  }));

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
              {loading ? '...' : patients.length}
            </Text>
          </Card>
        </Grid.Col>
        
        <Grid.Col span={{ base: 12, md: 4 }}>
          <Card shadow="sm" padding="lg" radius="md" withBorder>
            <Group justify="space-between" mb="xs">
              <Text fw={500}>Total Records</Text>
            </Group>
            <Text size="xl" fw="bold" c="green">
              {loading ? '...' : patients.length}
            </Text>
          </Card>
        </Grid.Col>
        
        <Grid.Col span={{ base: 12, md: 4 }}>
          <Card shadow="sm" padding="lg" radius="md" withBorder>
            <Group justify="space-between" mb="xs">
              <Text fw={500}>Status</Text>
            </Group>
            <Text size="xl" fw="bold" c={error ? "red" : "green"}>
              {error ? 'Error' : 'Ready'}
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