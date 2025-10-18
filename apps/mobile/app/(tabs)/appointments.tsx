import React from 'react';
import { ScrollView, StyleSheet } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { useRouter } from 'expo-router';
import { Text, Card, Surface, Button, List } from '@open-hims/mobile';

export default function AppointmentsScreen() {
  const router = useRouter();

  const appointments = [
    { id: 1, patient: 'John Doe', time: '09:00 AM', type: 'Consultation' },
    { id: 2, patient: 'Jane Smith', time: '10:30 AM', type: 'Follow-up' },
    { id: 3, patient: 'Bob Johnson', time: '02:00 PM', type: 'Check-up' },
  ];

  return (
    <>
      <StatusBar style="auto" />
      <Surface style={styles.container}>
        <Surface style={styles.header}>
          <Text variant="headlineMedium" style={styles.title}>
            Appointments
          </Text>
        </Surface>

        <ScrollView style={styles.content}>
          <Card style={styles.card}>
            <Card.Content>
              <Text variant="titleMedium" style={styles.cardTitle}>
                Today's Schedule
              </Text>
              
              {appointments.map((appointment) => (
                <List.Item
                  key={appointment.id}
                  title={appointment.patient}
                  description={`${appointment.time} - ${appointment.type}`}
                  left={(props) => <List.Icon {...props} icon="clock" />}
                  right={(props) => <List.Icon {...props} icon="chevron-right" />}
                  onPress={() => {
                    console.log('View appointment:', appointment.id);
                  }}
                  style={styles.listItem}
                />
              ))}
            </Card.Content>
          </Card>

          <Card style={styles.card}>
            <Card.Content>
              <Text variant="titleMedium" style={styles.cardTitle}>
                Quick Actions
              </Text>
              <Button
                mode="contained"
                onPress={() => console.log('Schedule appointment')}
                style={styles.actionButton}
              >
                Schedule New Appointment
              </Button>
              <Button
                mode="outlined"
                onPress={() => console.log('View calendar')}
                style={styles.actionButton}
              >
                View Full Calendar
              </Button>
            </Card.Content>
          </Card>
        </ScrollView>
      </Surface>
    </>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    padding: 20,
    backgroundColor: '#1976D2',
    paddingTop: 60,
  },
  title: {
    color: '#fff',
    fontWeight: 'bold',
  },
  content: {
    flex: 1,
    padding: 16,
  },
  card: {
    marginBottom: 16,
    elevation: 4,
  },
  cardTitle: {
    marginBottom: 16,
    color: '#1976D2',
  },
  listItem: {
    paddingVertical: 8,
  },
  actionButton: {
    marginBottom: 8,
  },
});