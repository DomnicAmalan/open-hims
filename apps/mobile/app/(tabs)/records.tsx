import React from 'react';
import { ScrollView, StyleSheet } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { Text, Card, Surface, Button, List } from '@open-hims/mobile';

export default function RecordsScreen() {
  const records = [
    { id: 1, patient: 'John Doe', type: 'Lab Report', date: '2024-01-15' },
    { id: 2, patient: 'Jane Smith', type: 'X-Ray', date: '2024-01-14' },
    { id: 3, patient: 'Bob Johnson', type: 'Blood Test', date: '2024-01-13' },
  ];

  return (
    <>
      <StatusBar style="auto" />
      <Surface style={styles.container}>
        <Surface style={styles.header}>
          <Text variant="headlineMedium" style={styles.title}>
            Medical Records
          </Text>
        </Surface>

        <ScrollView style={styles.content}>
          <Card style={styles.card}>
            <Card.Content>
              <Text variant="titleMedium" style={styles.cardTitle}>
                Recent Records
              </Text>
              
              {records.map((record) => (
                <List.Item
                  key={record.id}
                  title={`${record.patient} - ${record.type}`}
                  description={`Date: ${record.date}`}
                  left={(props) => <List.Icon {...props} icon="file-document" />}
                  right={(props) => <List.Icon {...props} icon="chevron-right" />}
                  onPress={() => console.log('View record:', record.id)}
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
                onPress={() => console.log('Add new record')}
                style={styles.actionButton}
              >
                Add New Record
              </Button>
              
              <Button
                mode="outlined"
                onPress={() => console.log('Search records')}
                style={styles.actionButton}
              >
                Search Records
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
