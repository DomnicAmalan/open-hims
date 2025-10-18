import React from 'react';
import { ScrollView, StyleSheet } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { Text, Card, Surface, Button, List } from '@open-hims/mobile';

export default function SettingsScreen() {
  return (
    <>
      <StatusBar style="auto" />
      <Surface style={styles.container}>
        <Surface style={styles.header}>
          <Text variant="headlineMedium" style={styles.title}>
            Settings
          </Text>
        </Surface>

        <ScrollView style={styles.content}>
          <Card style={styles.card}>
            <Card.Content>
              <Text variant="titleMedium" style={styles.cardTitle}>
                Account Settings
              </Text>
              
              <List.Item
                title="Profile"
                description="Update your profile information"
                left={(props) => <List.Icon {...props} icon="account" />}
                right={(props) => <List.Icon {...props} icon="chevron-right" />}
                onPress={() => console.log('Profile settings')}
                style={styles.listItem}
              />
              
              <List.Item
                title="Security"
                description="Change password and security settings"
                left={(props) => <List.Icon {...props} icon="shield" />}
                right={(props) => <List.Icon {...props} icon="chevron-right" />}
                onPress={() => console.log('Security settings')}
                style={styles.listItem}
              />
            </Card.Content>
          </Card>

          <Card style={styles.card}>
            <Card.Content>
              <Text variant="titleMedium" style={styles.cardTitle}>
                App Settings
              </Text>
              
              <List.Item
                title="Notifications"
                description="Configure notification preferences"
                left={(props) => <List.Icon {...props} icon="bell" />}
                right={(props) => <List.Icon {...props} icon="chevron-right" />}
                onPress={() => console.log('Notification settings')}
                style={styles.listItem}
              />
              
              <List.Item
                title="Data & Privacy"
                description="Manage your data and privacy settings"
                left={(props) => <List.Icon {...props} icon="database" />}
                right={(props) => <List.Icon {...props} icon="chevron-right" />}
                onPress={() => console.log('Privacy settings')}
                style={styles.listItem}
              />
            </Card.Content>
          </Card>

          <Card style={styles.card}>
            <Card.Content>
              <Text variant="titleMedium" style={styles.cardTitle}>
                Actions
              </Text>
              
              <Button
                mode="outlined"
                onPress={() => console.log('Export data')}
                style={styles.actionButton}
              >
                Export Data
              </Button>
              
              <Button
                mode="outlined"
                onPress={() => console.log('Sign out')}
                style={[styles.actionButton, styles.signOutButton]}
                textColor="#d32f2f"
              >
                Sign Out
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
  signOutButton: {
    borderColor: '#d32f2f',
  },
});