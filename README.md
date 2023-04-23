# Inspiration

The inspiration behind CKBSafe comes from the growing need for enhanced security, reliability, and collaboration in the rapidly evolving world of decentralized finance and digital assets.

CKBSafe addresses these concerns by harnessing the power of multi-signature functionality, requiring multiple authorized signatures to approve a transaction. This added layer of security not only protects valuable assets but also promotes collaborative decision-making among stakeholders.

By hosting the multi-signature dApp using the Sia decentralized storage platform, we are decentralizing the entire project and ensuring higher levels of security, privacy, and resilience. In essence, CKBSafe revolutionizes the way users transact and collaborate on the Nervos CKB blockchain, fostering trust, confidence, and innovation in the decentralized world.

# What it does 

Multi-signature functionality, like CKBSafe, brings several important features and benefits to a blockchain like Nervos CKB, enhancing security, collaboration, and flexibility in transaction management:
- Enhanced Security: Multi-signature mechanisms require a predetermined number of authorized signatures for approving a transaction, reducing the risks of single points of failure, unauthorized access, and asset theft.
- Collaborative Management: Multi-signature functionality empowers multiple stakeholders to jointly manage assets or make decisions in various contexts, such as business partnerships, family trusts, or escrow services, fostering collaboration and transparency.
- Flexible Authorization: Multi-signature solutions offer customizable authorization schemes, allowing users to define the required threshold of signatures for transaction approval. This adaptability caters to a wide range of use cases and requirements.

By introducing multi-signature functionality, CKBSafe significantly improves the way users transact and collaborate on the Nervos CKB blockchain, promoting trust, confidence, and innovation in the decentralized world.

By leveraging Sia's decentralized storage platform to host the multi-signature dApp, we enhance the project's overall security, privacy, and resilience. We deployed it on skynet so that a merchant doesn't have to worry about the transaction infra going down if centralized services like AWS go down.

This decentralized approach aligns with the core principles of blockchain technology, ensuring the multi-signature dApp is more resistant to potential attacks, data loss, or censorship.

# What we learned 

Throughout our research on the Nervos CKB blockchain, we gained valuable insights into its unique architecture. The UTXO model of the Nervos CKB blockchain is an enhanced version of Bitcoin's UTXO approach, offering improved security and scalability. Transactions create, update, or consume cells, and the model enables parallel processing, ensuring deterministic outcomes while providing better auditability and reducing the risk of double-spending.

Nervos CKB leverages the RISC-V instruction set to compile smart contracts from various languages into binary files, enabling a versatile development ecosystem. Rust stood out as a popular choice due to its strong safety guarantees, concurrency support, and high-performance capabilities.

Leveraging Rust's expressive type system and pattern matching, we successfully developed and deployed the CKBSafe multi-signature contract. By harnessing Rust's powerful capabilities, we were able to create a reliable and secure solution for managing transactions on the Nervos CKB blockchain.

We also learned that Sia stores data on a global, peer-to-peer network, distributing files across multiple nodes. This decentralized approach eliminates reliance on centralized storage providers, mitigating risks of single points of failure, data breaches, or censorship.

By exploring and understanding Sia's capabilities during the development of the project, we gained valuable insights into how decentralized storage can contribute to improved security, privacy, and resilience for our multi-signature dApp.

# Challenges we faced

During the development of CKBSafe, we faced several challenges, including limited resources and unfamiliarity with Rust. The Nervos CKB blockchain's architecture, being significantly different from the Ethereum ecosystem, posed a steep learning curve. 

CKB's unique design philosophy, consensus mechanism, and smart contract execution model required us to invest a considerable amount of time and effort into research and understanding the underlying architecture. CKB's distinct approach, such as the use of the UTXO model instead of account-based model and different smart contract languages, demanded that we rethink our development strategies and learn new paradigms.

Additionally, our initial failure to discover comprehensive documentation and resources early on left us with limited time for development. Despite these hurdles, our team persevered, adapted to the new environment, and successfully tackled the complexities of Rust and the Nervos CKB blockchain to create a valuable multi-signature solution.

We faced several challenges during the integration of our project. Thoroughly testing and debugging the frontend and smart contract interactions on the CKB network was time-consuming, as we had to account for various edge cases and potential issues that could arise during transactions.

# What's next for CKBSafe

Adding more features and support to the Safe and making it production ready for use case. 
