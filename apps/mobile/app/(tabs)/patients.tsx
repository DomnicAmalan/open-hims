import React from 'react';
import { ScrollView, StyleSheet } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { useRouter } from 'expo-router';
import { Text, Card, Surface, Button, Searchbar, List } from '@open-hims/mobile';

export default function PatientsScreen() {
  const router = useRouter();
  const [searchQuery, setSearchQuery] = React.useState('');

  const patients = [
    { id: 1, name: 'John Doe', age: 45, condition: 'Hypertension' },
    { id: 2, name: 'Jane Smith', age: 32, condition: 'Diabetes' },
    { id: 3, name: 'Bob Johnson', age: 28, condition: 'Asthma' },
  ];

  return (
    <>
      <StatusBar style="auto" />
      <Surface style={styles.container}>
        <Surface style={styles.header}>
          <Text variant="headlineMedium" style={styles.title}>
            Patients
          </Text>
        </Surface>

        <ScrollView style={styles.content}>
          <Searchbar
            placeholder="Search patients..."
            onChangeText={setSearchQuery}
            value={searchQuery}
            style={styles.searchBar}
          />

          <Card style={styles.card}>
            <Card.Content>
              <Text variant="titleMedium" style={styles.cardTitle}>
                Patient List
              </Text>
              
              {patients.map((patient) => (
                <List.Item
                  key={patient.id}
                  title={patient.name}
                  description={`Age: ${patient.age} | Condition: ${patient.condition}`}
                  left={(props) => <List.Icon {...props} icon="account" />}
                  right={(props) => <List.Icon {...props} icon="chevron-right" />}
                  onPress={() => {
                    // Navigate to patient details
                    console.log('Navigate to patient:', patient.id);
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
                onPress={() => console.log('Add new patient')}
                style={styles.actionButton}
              >
                Add New Patient
              </Button>
              <Button
                mode="outlined"
                onPress={() => console.log('Import patients')}
                style={styles.actionButton}
              >
                Import Patients
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
  searchBar: {
    marginBottom: 16,
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