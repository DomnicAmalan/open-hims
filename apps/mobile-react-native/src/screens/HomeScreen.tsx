import React from 'react';
import { StyleSheet } from 'react-native';
import { useSelector, useDispatch } from 'react-redux';
import { 
  View,
  ScrollView,
  Surface,
  Text,
  Card,
  Button,
  ActivityIndicator,
  TouchableRipple,
} from '@open-hims/mobile';
import { 
  fetchPatients,
  selectAllPatients,
  selectPatientsLoading,
  RootState,
  AppDispatch
} from '@open-hims/store';

export default function HomeScreen({ navigation }: any) {
  const dispatch = useDispatch<AppDispatch>();
  const patients = useSelector((state: RootState) => selectAllPatients(state));
  const patientsState = useSelector((state: RootState) => selectPatientsLoading(state));

  const handleViewPatients = () => {
    navigation.navigate('PatientList');
  };

  const handleLoadPatients = () => {
    dispatch(fetchPatients({ page: 1, pageSize: 20 }));
  };

  return (
    <ScrollView style={styles.container}>
      <Surface style={styles.header}>
        <Text variant="headlineMedium" style={styles.title}>Open HIMS Mobile</Text>
        <Text variant="bodyMedium" style={styles.subtitle}>Healthcare Information Management System</Text>
      </Surface>

      <View style={styles.statsContainer}>
        <Card style={styles.statCard}>
          <Card.Content>
            <Text variant="titleLarge" style={styles.statNumber}>{patients.length}</Text>
            <Text variant="bodySmall" style={styles.statLabel}>Total Patients</Text>
          </Card.Content>
        </Card>
        
        <Card style={styles.statCard}>
          <Card.Content>
            <Text variant="titleLarge" style={styles.statNumber}>12</Text>
            <Text variant="bodySmall" style={styles.statLabel}>Active Cases</Text>
          </Card.Content>
        </Card>
        
        <Card style={styles.statCard}>
          <Card.Content>
            <Text variant="titleLarge" style={styles.statNumber}>5</Text>
            <Text variant="bodySmall" style={styles.statLabel}>Pending Tasks</Text>
          </Card.Content>
        </Card>
      </View>

      <View style={styles.actionsContainer}>
        <Button 
          mode="contained"
          style={styles.actionButton} 
          onPress={handleViewPatients}
        >
          View Patients
        </Button>

        <Button 
          mode="contained"
          style={styles.actionButton} 
          onPress={handleLoadPatients}
          disabled={patientsState.fetchPatients}
          loading={patientsState.fetchPatients}
        >
          {patientsState.fetchPatients ? 'Loading...' : 'Refresh Data'}
        </Button>

        <Button 
          mode="contained"
          style={styles.actionButton}
          onPress={() => navigation.navigate('Settings')}
        >
          Settings
        </Button>
      </View>

      <View style={styles.featuresContainer}>
        <Text variant="titleLarge" style={styles.sectionTitle}>Features</Text>
        
        <Card style={styles.featureItem}>
          <Card.Content>
            <Text variant="titleMedium" style={styles.featureTitle}>🏥 Patient Management</Text>
            <Text variant="bodyMedium" style={styles.featureDescription}>
              Comprehensive patient records with FHIR compliance
            </Text>
          </Card.Content>
        </Card>

        <Card style={styles.featureItem}>
          <Card.Content>
            <Text variant="titleMedium" style={styles.featureTitle}>🔒 Security & Compliance</Text>
            <Text variant="bodyMedium" style={styles.featureDescription}>
              HIPAA, GDPR compliant with audit logging
            </Text>
          </Card.Content>
        </Card>

        <Card style={styles.featureItem}>
          <Card.Content>
            <Text variant="titleMedium" style={styles.featureTitle}>🌍 Multi-Country Support</Text>
            <Text variant="bodyMedium" style={styles.featureDescription}>
              Support for various healthcare regulations
            </Text>
          </Card.Content>
        </Card>

        <Card style={styles.featureItem}>
          <Card.Content>
            <Text variant="titleMedium" style={styles.featureTitle}>📱 Cross-Platform</Text>
            <Text variant="bodyMedium" style={styles.featureDescription}>
              Shared components across web, mobile, and desktop
            </Text>
          </Card.Content>
        </Card>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    backgroundColor: '#0066cc',
    padding: 20,
    paddingTop: 40,
    alignItems: 'center',
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: 'white',
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 16,
    color: '#e6f2ff',
    textAlign: 'center',
  },
  statsContainer: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    marginTop: -30,
    marginHorizontal: 20,
  },
  statCard: {
    backgroundColor: 'white',
    borderRadius: 12,
    padding: 16,
    alignItems: 'center',
    flex: 1,
    marginHorizontal: 4,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  statNumber: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#0066cc',
  },
  statLabel: {
    fontSize: 12,
    color: '#666',
    marginTop: 4,
    textAlign: 'center',
  },
  actionsContainer: {
    margin: 20,
  },
  actionButton: {
    backgroundColor: '#0066cc',
    borderRadius: 8,
    padding: 16,
    marginBottom: 12,
    alignItems: 'center',
  },
  actionButtonText: {
    color: 'white',
    fontSize: 16,
    fontWeight: '600',
  },
  featuresContainer: {
    margin: 20,
  },
  sectionTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 16,
  },
  featureItem: {
    backgroundColor: 'white',
    borderRadius: 8,
    padding: 16,
    marginBottom: 12,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.1,
    shadowRadius: 2,
    elevation: 2,
  },
  featureTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#333',
    marginBottom: 4,
  },
  featureDescription: {
    fontSize: 14,
    color: '#666',
    lineHeight: 20,
  },
});