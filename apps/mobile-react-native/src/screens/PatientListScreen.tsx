import React from 'react';
import { StyleSheet, FlatList } from 'react-native';
import { useSelector, useDispatch } from 'react-redux';
import { 
  View,
  Text,
  Card,
  ActivityIndicator,
  Surface,
  TouchableRipple,
} from '@open-hims/mobile';
import { RootState, selectPatient } from '@open-hims/store';

export default function PatientListScreen({ navigation }: any) {
  const dispatch = useDispatch();
  const { patients, loading, searchResults } = useSelector((state: RootState) => state.patients);
  
  const displayPatients = searchResults.length > 0 ? searchResults : patients;

  const handlePatientPress = (patient: any) => {
    dispatch(selectPatient(patient));
    navigation.navigate('PatientDetail', { patientId: patient.id });
  };

  const renderPatientItem = ({ item: patient }: { item: any }) => (
    <Card style={styles.patientCard}>
      <TouchableRipple onPress={() => handlePatientPress(patient)}>
        <Card.Content>
          <View style={styles.patientHeader}>
            <Text variant="titleMedium" style={styles.patientName}>
              {patient.firstName} {patient.lastName}
            </Text>
            <Text variant="labelMedium" style={styles.patientMrn}>MRN: {patient.mrn}</Text>
          </View>
          
          <View style={styles.patientDetails}>
            <Text variant="bodyMedium" style={styles.patientDetail}>
              Age: {calculateAge(patient.dateOfBirth)} • {capitalizeFirst(patient.gender)}
            </Text>
            <Text variant="bodyMedium" style={styles.patientDetail}>
              📧 {patient.email || 'No email'}
            </Text>
            <Text variant="bodyMedium" style={styles.patientDetail}>
              📞 {patient.phone || 'No phone'}
            </Text>
          </View>

          {patient.address && (
            <Text variant="bodySmall" style={styles.patientAddress}>
              📍 {patient.address.city}, {patient.address.state}
            </Text>
          )}
        </Card.Content>
      </TouchableRipple>
    </Card>
  );

  if (loading.fetchPatients) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator animating={true} size="large" />
        <Text variant="bodyLarge" style={styles.loadingText}>Loading patients...</Text>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <Surface style={styles.header}>
        <Text variant="titleLarge" style={styles.headerTitle}>Patients ({displayPatients.length})</Text>
      </Surface>

      {displayPatients.length === 0 ? (
        <View style={styles.emptyContainer}>
          <Text variant="titleMedium" style={styles.emptyText}>No patients found</Text>
          <Text variant="bodyMedium" style={styles.emptySubtext}>
            Add patients or refresh to see data
          </Text>
        </View>
      ) : (
        <FlatList
          data={displayPatients}
          renderItem={renderPatientItem}
          keyExtractor={(item) => item.id}
          contentContainerStyle={styles.listContainer}
          showsVerticalScrollIndicator={false}
        />
      )}
    </View>
  );
}

// Helper functions
function calculateAge(dateOfBirth: string): number {
  const today = new Date();
  const birthDate = new Date(dateOfBirth);
  let age = today.getFullYear() - birthDate.getFullYear();
  const monthDiff = today.getMonth() - birthDate.getMonth();
  
  if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
    age--;
  }
  
  return age;
}

function capitalizeFirst(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    backgroundColor: '#0066cc',
    padding: 16,
    alignItems: 'center',
  },
  headerTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: 'white',
  },
  listContainer: {
    padding: 16,
  },
  patientCard: {
    backgroundColor: 'white',
    borderRadius: 12,
    padding: 16,
    marginBottom: 12,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  patientHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
  },
  patientName: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    flex: 1,
  },
  patientMrn: {
    fontSize: 12,
    color: '#0066cc',
    fontWeight: '600',
    backgroundColor: '#e6f2ff',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 4,
  },
  patientDetails: {
    marginBottom: 8,
  },
  patientDetail: {
    fontSize: 14,
    color: '#666',
    marginBottom: 2,
  },
  patientAddress: {
    fontSize: 12,
    color: '#999',
    fontStyle: 'italic',
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  loadingText: {
    fontSize: 16,
    color: '#666',
  },
  emptyContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  emptyText: {
    fontSize: 18,
    color: '#666',
    marginBottom: 8,
  },
  emptySubtext: {
    fontSize: 14,
    color: '#999',
    textAlign: 'center',
  },
});