import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
	id("org.springframework.boot") version "3.4.5"
	id("io.spring.dependency-management") version "1.1.7"
	kotlin("jvm") version "2.1.20"
	kotlin("plugin.spring") version "2.1.20"
	kotlin("plugin.jpa") version "2.1.20"
	kotlin("plugin.serialization") version "2.1.20"
}

group = "com.squireofsoftware"
version = "0.0.1-SNAPSHOT"

java {
	sourceCompatibility = JavaVersion.VERSION_21
	toolchain {
		languageVersion = JavaLanguageVersion.of(21)
	}
}

repositories {
	mavenCentral()
}

dependencyManagement {
	imports {
//		mavenBom( "org.springframework.cloud:spring-cloud-dependencies:2024.0.1")
//		mavenBom("org.springframework.cloud:spring-cloud-sleuth-otel-dependencies:1.1.4")
	}
}

dependencies {
	implementation("org.springframework.boot:spring-boot-starter")
	implementation("org.springframework.boot:spring-boot-starter-web")
	implementation("org.springframework.boot:spring-boot-starter-actuator")
	implementation("org.jetbrains.kotlin:kotlin-reflect")
	testImplementation("org.springframework.boot:spring-boot-starter-test")
	implementation("org.springframework.boot:spring-boot-starter-data-jpa")
	implementation("com.h2database:h2:2.3.232")
	implementation(platform("com.netflix.graphql.dgs:graphql-dgs-platform-dependencies:latest.release"))
	implementation(platform("io.micrometer:micrometer-bom:1.14.6"))
	implementation("io.micrometer:micrometer-registry-prometheus")
	implementation("com.netflix.graphql.dgs:dgs-starter")
	implementation("org.springframework.kafka:spring-kafka:4.0.0-M2")
	implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.8.1")
	implementation("io.opentelemetry:opentelemetry-exporter-otlp")
	testImplementation("io.mockk:mockk:1.14.2")
	testImplementation("com.squareup.okhttp3:mockwebserver:4.12.0")
}

tasks.withType<Test> {
	useJUnitPlatform()
}
