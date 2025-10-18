import React from 'react';
import { StyleSheet, View } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { Text, Card, Surface } from '@open-hims/mobile';
import { useRouter } from 'expo-router';

export default function DashboardScreen() {
  const router = useRouter();

  return (
    <View style={styles.container}>
      <Surface style={styles.header} elevation={2}>
        <Text variant="headlineSmall" style={styles.headerText}>
          📊 Dashboard
        </Text>
      </Surface>

      <Card style={styles.card}>
        <Card.Content>
          <Text variant="titleMedium">Welcome to HIMS Dashboard</Text>
          <Text variant="bodyMedium" style={styles.description}>
            View your healthcare analytics and system overview
          </Text>
        </Card.Content>
      </Card>

      <Card style={styles.card}>
        <Card.Content>
          <Text variant="titleMedium">Quick Stats</Text>
          <Text variant="bodyMedium" style={styles.description}>
            • Active Patients: 1,234
          </Text>
          <Text variant="bodyMedium" style={styles.description}>
            • Today's Appointments: 45
          </Text>
          <Text variant="bodyMedium" style={styles.description}>
            • System Status: Operational
          </Text>
        </Card.Content>
      </Card>

      <StatusBar style="auto" />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
    padding: 16,
    paddingTop: 60,
  },
  header: {
    padding: 20,
    borderRadius: 12,
    marginBottom: 16,
    alignItems: 'center',
  },
  headerText: {
    fontWeight: 'bold',
  },
  card: {
    marginBottom: 16,
  },
  description: {
    marginTop: 8,
    opacity: 0.8,
  },
});