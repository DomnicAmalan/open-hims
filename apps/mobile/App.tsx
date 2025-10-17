import React from 'react';
import { StatusBar } from 'expo-status-bar';
import { StyleSheet, View, ImageBackground, Dimensions } from 'react-native';
import { 
  HimsPaperProvider, 
  HimsGlassCard, 
  HimsGlassHeader,
  HimsGlassActionBar,
  HimsGlassInfoPanel,
  Text, 
  Button 
} from '@open-hims/mobile';

const { width, height } = Dimensions.get('window');

export default function App() {
  return (
    <HimsPaperProvider theme="auto">
      <View style={styles.container}>
        {/* Background with gradient effect */}
        <ImageBackground
          source={{
            uri: 'https://images.unsplash.com/photo-1576091160399-112ba8d25d1f?w=400&h=800&fit=crop&auto=format'
          }}
          style={styles.backgroundImage}
          blurRadius={2}
        >
          {/* Main Glass Header */}
          <HimsGlassHeader 
            title="🏥 Open HIMS" 
            subtitle="Healthcare Information Management System"
            style={styles.mainHeader}
          />

          {/* Features Glass Card */}
          <HimsGlassCard style={styles.featuresCard}>
            <Text variant="titleMedium" style={styles.featuresTitle}>
              Healthcare Standards:
            </Text>
            <Text variant="bodySmall" style={styles.feature}>
              • FHIR R4/R5 for data exchange
            </Text>
            <Text variant="bodySmall" style={styles.feature}>
              • HIPAA & GDPR compliance
            </Text>
            <Text variant="bodySmall" style={styles.feature}>
              • HL7v2 & DICOM support
            </Text>
            <Text variant="bodySmall" style={styles.feature}>
              • ABDM integration ready
            </Text>
          </HimsGlassCard>

          {/* Info Panel */}
          <HimsGlassInfoPanel priority="medium" style={styles.infoPanel}>
            <Text variant="bodyMedium" style={styles.infoText}>
              🚀 Rust-powered core with React Native UI
            </Text>
          </HimsGlassInfoPanel>

          {/* Action Glass Card */}
          <HimsGlassActionBar style={styles.actionBar}>
            <Button 
              mode="contained" 
              style={styles.button}
              buttonColor="rgba(33, 150, 243, 0.8)"
              textColor="#fff"
            >
              Get Started
            </Button>
            <Button 
              mode="outlined" 
              style={styles.secondaryButton}
              textColor="#1976D2"
            >
              Documentation
            </Button>
          </HimsGlassActionBar>
        </ImageBackground>
        <StatusBar style="light" />
      </View>
    </HimsPaperProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  backgroundImage: {
    flex: 1,
    width: '100%',
    height: '100%',
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  mainHeader: {
    marginBottom: 24,
    width: '100%',
    maxWidth: 400,
  },
  featuresCard: {
    marginBottom: 16,
    width: '100%',
    maxWidth: 400,
  },
  featuresTitle: {
    marginBottom: 12,
    fontWeight: 'bold',
    color: '#333',
  },
  feature: {
    marginBottom: 6,
    marginLeft: 8,
    color: '#555',
  },
  infoPanel: {
    marginBottom: 20,
    width: '100%',
    maxWidth: 400,
  },
  infoText: {
    textAlign: 'center',
    color: '#333',
  },
  actionBar: {
    width: '100%',
    maxWidth: 400,
    flexDirection: 'row',
    justifyContent: 'space-around',
  },
  button: {
    flex: 1,
    marginHorizontal: 8,
    paddingHorizontal: 24,
  },
  secondaryButton: {
    flex: 1,
    marginHorizontal: 8,
  },
});
