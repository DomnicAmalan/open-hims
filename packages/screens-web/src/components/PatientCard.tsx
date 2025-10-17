import React from 'react';
import { Card, Group, Text, Badge, Avatar } from '@mantine/core';
import type { UIPatient } from '@open-hims/types';

interface PatientCardProps {
  patient: UIPatient;
  onClick?: () => void;
}

export function PatientCard({ patient, onClick }: PatientCardProps) {
  const getInitials = (firstName?: string, lastName?: string) => {
    return `${firstName?.[0] || ''}${lastName?.[0] || ''}`.toUpperCase();
  };

  return (
    <Card
      shadow="sm"
      padding="lg"
      radius="md"
      withBorder
      style={{ cursor: onClick ? 'pointer' : 'default' }}
      onClick={onClick}
    >
      <Group>
        <Avatar color="blue" radius="xl">
          {getInitials(patient.firstName, patient.lastName)}
        </Avatar>
        
        <div style={{ flex: 1 }}>
          <Text fw={500}>
            {patient.firstName} {patient.lastName}
          </Text>
          <Text size="sm" c="dimmed">
            MRN: {patient.mrn}
          </Text>
          {patient.email && (
            <Text size="sm" c="dimmed">
              {patient.email}
            </Text>
          )}
        </div>
        
        <div>
          <Badge color="blue" size="sm">
            {patient.gender || 'Unknown'}
          </Badge>
          {patient.dateOfBirth && (
            <Text size="xs" c="dimmed" mt={4}>
              DOB: {patient.dateOfBirth}
            </Text>
          )}
        </div>
      </Group>
    </Card>
  );
}