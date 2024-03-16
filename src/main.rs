use leptos::*;

#[component]
fn App() -> impl IntoView {
    // let (count, set_count) = create_signal(0);

    view! {
       <>
        <header>
            <h1>{"🌟 Mohammadreza Rahimiangolkhandani 🌟"}</h1>
        </header>
        <div class="container">
            <section>
                <h2>{"About Me"}</h2>
                <ul>
                    <li>{"📍 Location: Vancouver, BC"}</li>
                    <li>{"👨‍💻 GitHub: "}<a href="https://github.com/golkhandani">{"golkhandani"}</a></li>
                    <li>{"👔 LinkedIn: "}<a href="https://www.linkedin.com/in/golkhandani">{"golkhandani"}</a></li>
                    <li>{"🚀 Dev.to: "}<a href="https://dev.to/golkhandani">{"golkhandani"}</a></li>
                </ul>
                <h2>{"Work Experiences"}</h2>
                <div class="work">
                    <h3>{"Full Stack Developer (2022-12 - 2024-01)"}</h3>
                    <ul>
                        <li><strong>{"Company:"}</strong>{" SkyHive Technologies Inc."}</li>
                        <li><strong>{"Location:"}</strong>{" Vancouver, BC, Canada"}</li>
                        <li><strong>{"Description:"}</strong>
                            <ul>
                                <li>{"Developed performant web and mobile SaaS applications using Flutter/ReactJS."}</li>
                                <li>{"Implemented highly customized theming, animations, and widget systems."}</li>
                                <li>{"Built and maintained APIs with Node.js and GraphQL."}</li>
                                <li>{"Utilized Kafka and RabbitMQ to maintain a distributed workload."}</li>
                                <li>{"Interfaced with 3rd party APIs and internal APIs."}</li>
                                <li>{"Collaborated with project management and design teams applying Agile best practices. 🚀"}</li>
                            </ul>
                        </li>
                    </ul>
                </div>
                <div class="work">
                    <h3>{"Software Developer (2021-10 - 2022-12)"}</h3>
                    <ul>
                        <li><strong>{"Company:"}</strong>{" Plasmatic Technologies Inc."}</li>
                        <li><strong>{"Location:"}</strong>{" Vancouver, BC, Canada"}</li>
                        <li><strong>{"Description:"}</strong>
                            <ul>
                                <li>{"Built backend services for SaaS applications using Node.js, Serverless framework, and GraphQL."}</li>
                                <li>{"Implemented a user-based recommendation system."}</li>
                                <li>{"Optimized IoT communication services and authentication systems."}</li>
                                <li>{"Designed software and cloud architectures and utilized Git/Bitbucket, Jira, Docker, CI/CD. 💻"}</li>
                            </ul>
                        </li>
                    </ul>
                </div>
                <div class="work">
                    <h3>{"Backend Developer (2020-03 - 2021-09)"}</h3>
                    <ul>
                        <li><strong>{"Company:"}</strong>{" Sparlus(Supnex) Company"}</li>
                        <li><strong>{"Location:"}</strong>{" Tehran, Tehran, Iran"}</li>
                        <li><strong>{"Description:"}</strong>
                            <ul>
                                <li>{"Designed SaaS software architecture using Node.js, Nest.js, MongoDB, PostgreSQL, and Redis."}</li>
                                <li>{"Contributed to in-house packages and designed Elasticsearch Schema."}</li>
                                <li>{"Built an MVC application for a restaurant menu. 🍔"}</li>
                            </ul>
                        </li>
                    </ul>
                </div>
                <div class="work">
                    <h3>{"Full Stack Developer (2016-12 - 2020-03)"}</h3>
                    <ul>
                        <li><strong>{"Company:"}</strong>{" Peeyade"}</li>
                        <li><strong>{"Location:"}</strong>{" Tehran, Tehran, Iran"}</li>
                        <li><strong>{"Description:"}</strong>
                            <ul>
                                <li>{"Developed and maintained urban discovery applications using Express.js, Nest.js, and TypeScript."}</li>
                                <li>{"Utilized Angular.js/2+ and Socket.io for internal admin panel."}</li>
                                <li>{"Developed real-time gamification chatbot and eCommerce app."}</li>
                                <li>{"Integrated Logstash for monitoring and log purposes. 🛠️"}</li>
                            </ul>
                        </li>
                    </ul>
                </div>
                <div class="work">
                    <h2>{"Open Source Projects"}</h2>
                    <ul>
                        <li>{"Image-hero: Image manipulator using sharp API "}<a href="link">{"Git"}</a></li>
                        <li>{"Recive: Your personal tour guide in B.C "}<a href="link">{"Git"}</a></li>
                        <li>{"Cookbook for student: Recipe application based on the mealdb API "}<a href="link">{"Git"}</a></li>
                        <li>{"mysql2-nestjs: npm package for MySQL2 integration with NestJS Framework "}<a href="link">{"Git"}</a></li>
                        <li>{"Fastest-validator-nestjs: npm for a Class-based validation module using decorators "}<a href="link">{"Git"}</a></li>
                    </ul>
                </div>
                <div class="work">
                    <h2>{"Interests"}</h2>
                    <ul>
                        <li>{"JavaScript (TypeScript)"}</li>
                        <li>{"Dart (Flutter)"}</li>
                        <li>{"DBMS: MySQL, PostgreSQL"}</li>
                        <li>{"NoSQL: MongoDB, Redis"}</li>
                        <li>{"Elasticsearch"}</li>
                        <li>{"Programming: Object-oriented, Functional, Socket, Async"}</li>
                        <li>{"Architecture: SaaS, Cloud-native, Microservice"}</li>
                        <li>{"Docker"}</li>
                        <li>{"GraphQL (Apollo Server)"}</li>
                    </ul>
                </div>
            </section>
        </div>
       </>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
