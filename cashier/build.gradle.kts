import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
	id("org.springframework.boot") version "3.1.4"
	id("io.spring.dependency-management") version "1.1.3"
	kotlin("jvm") version "1.8.22"
	kotlin("plugin.spring") version "1.8.22"
	kotlin("plugin.jpa") version "1.9.10"
	kotlin("plugin.serialization") version "1.9.0"
}

group = "com.squireofsoftware"
version = "0.0.1-SNAPSHOT"

java {
	sourceCompatibility = JavaVersion.VERSION_17
}

repositories {
	mavenCentral()
}

dependencies {
	implementation("org.springframework.boot:spring-boot-starter")
	implementation("org.springframework.boot:spring-boot-starter-web")
	implementation("org.jetbrains.kotlin:kotlin-reflect")
	testImplementation("org.springframework.boot:spring-boot-starter-test")
	implementation("org.springframework.boot:spring-boot-starter-data-jpa")
	implementation("com.h2database:h2:2.2.224")
	implementation(platform("com.netflix.graphql.dgs:graphql-dgs-platform-dependencies:latest.release"))
	implementation("com.netflix.graphql.dgs:graphql-dgs-spring-boot-starter")
	implementation("org.springframework.kafka:spring-kafka:3.0.11")
	implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.0")
	implementation("org.springframework.cloud:spring-cloud-starter-sleuth") {
		exclude("spring-cloud-sleuth-brave")
	}
	implementation("org.springframework.cloud:spring-cloud-sleuth-otel-autoconfigure")
	implementation("io.opentelemetry:opentelemetry-exporter-otlp")
	implementation(platform("org.springframework.cloud:spring-cloud-dependencies:2021.0.5"))
	implementation(platform("org.springframework.cloud:spring-cloud-sleuth-otel-dependencies:1.1.2"))
	testImplementation("io.mockk:mockk:1.13.9")
	testImplementation("com.squareup.okhttp3:mockwebserver:4.12.0")

}

tasks.withType<KotlinCompile> {
	kotlinOptions {
		freeCompilerArgs += "-Xjsr305=strict"
		jvmTarget = "17"
	}
}

tasks.withType<Test> {
	useJUnitPlatform()
}
