import React from 'react';
import { StatusBar } from 'expo-status-bar';
import { StyleSheet, View } from 'react-native';
import { HimsPaperProvider, Text, Surface, Button } from '@open-hims/mobile';

export default function App() {
  return (
    <HimsPaperProvider theme="auto">
      <View style={styles.container}>
        <Surface style={styles.surface}>
          <Text variant="headlineMedium" style={styles.title}>
            🏥 Open HIMS Mobile
          </Text>
          <Text variant="bodyMedium" style={styles.subtitle}>
            Healthcare Information Management System
          </Text>
          
          <View style={styles.featuresContainer}>
            <Text variant="titleMedium" style={styles.featuresTitle}>
              Features:
            </Text>
            <Text variant="bodySmall" style={styles.feature}>
              • Patient Management with FHIR compliance
            </Text>
            <Text variant="bodySmall" style={styles.feature}>
              • HIPAA & GDPR compliant security
            </Text>
            <Text variant="bodySmall" style={styles.feature}>
              • Cross-platform React Native app
            </Text>
          </View>
          
          <Button mode="contained" style={styles.button}>
            Get Started
          </Button>
        </Surface>
        <StatusBar style="auto" />
      </View>
    </HimsPaperProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  surface: {
    padding: 24,
    borderRadius: 16,
    alignItems: 'center',
    width: '100%',
    maxWidth: 400,
  },
  title: {
    marginBottom: 8,
    textAlign: 'center',
  },
  subtitle: {
    marginBottom: 24,
    textAlign: 'center',
    opacity: 0.7,
  },
  featuresContainer: {
    marginBottom: 24,
    width: '100%',
  },
  featuresTitle: {
    marginBottom: 12,
    fontWeight: 'bold',
  },
  feature: {
    marginBottom: 6,
    marginLeft: 8,
  },
  button: {
    marginTop: 8,
    paddingHorizontal: 24,
  },
});
