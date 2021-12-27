pipeline {

    agent {
        label 'docker'
    }

    environment {
        CONTAINER_REGISTRY = credentials("container-registry")
        BUILD_TAG = "${CONTAINER_REGISTRY_USR}/mattiarubinicom:latest-${PROCESS_ARCHITECTURE}"
    }

    stages {
        stage ('Build') {
            steps {

                script {

                    def mattiarubinicom_container = docker.build("${BUILD_TAG}", '-f mattiarubinicom.Dockerfile ./container/')

                }
                
            }
        }
        stage ('Test'){
            steps {

                script {

                    docker.image("${BUILD_TAG}").inside {

                        sh 'echo tests pending'

                    }

                }
                //sh 'echo "yet to be defined"'

            }
        }
        stage ('Upload') {
            steps{

                script {

                    //  Use this code if using a registry that is not the one configured in 
                    //      http://your.jenkins.server.url/configure
                    //  in the section 
                    //      Declarative Pipeline (Docker)

                    /*docker.withRegistry('https://index.docker.io/v1/', 'container-registry') {

                        def mattiarubinicom_container = docker.image("${BUILD_TAG}")
                        mattiarubinicom_container.push()

                    }*/

                    def mattiarubinicom_container = docker.image("${BUILD_TAG}")
                    mattiarubinicom_container.push()

                }
                //sh 'docker login -u ${CONTAINER_REGISTRY_USR} -p ${CONTAINER_REGISTRY_PSW} index.docker.io && docker push $BUILD_TAG'

            }
        }
    }

}