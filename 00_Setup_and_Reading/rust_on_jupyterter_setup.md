# **Complete Guide: Running Rust in Jupyter Notebook with VS Code**

This guide walks you through the process of setting up Rust as a kernel for Jupyter notebooks, allowing you to run and interact with Rust code directly in Jupyter notebooks. It also covers how to do this within **Visual Studio Code** (VS Code) for an integrated coding experience. 

This repository is dedicated to **Rust developers**, **Data Scientists**, and **AI/ML Engineers** who want to learn Rust with a Jupyter notebook experience. 

### **Prerequisites:**

Ensure you have the following installed before you begin:

1. **Rust** (via `rustup`)
2. **Jupyter Notebook**
3. **VS Code** (with relevant extensions for Rust and Jupyter)

---

### **Step 1: Install Rust**
   *Follow the installation guide on the rust_installer.md file*

### **Step 2: Install `evcxr_jupyter` (Rust Kernel for Jupyter)**

To run Rust in Jupyter, you need to install `evcxr_jupyter`, the Rust kernel for Jupyter notebooks:

1. In your terminal, run the following command to install `evcxr_jupyter`:

   ```bash
   cargo install evcxr_jupyter
   ```

2. After installation, set up the kernel with the following command:

   ```bash
   evcxr_jupyter --install
   ```

This will install the necessary files to run the Rust kernel in Jupyter notebooks.


### **Step 3: Install Jupyter Notebook**

If Jupyter Notebook isn't installed, you can do so via `pip`:

1. Install Jupyter Notebook using `pip`:

   ```bash
   pip install notebook
   ```


### **Step 4: Verify Installation**

Check if the Rust kernel is successfully installed by running the following command:

1. In your terminal, type:

   ```bash
   jupyter kernelspec list
   ```

2. You should see an entry for the Rust kernel like this:

   ```
   Available kernels:
     rust                    /home/yourusername/.local/share/jupyter/kernels/rust
     python3                 /usr/local/share/jupyter/kernels/python3
   ```


### **Step 5: Use Rust in Jupyter Notebook via VS Code**

You can run Jupyter notebooks within **VS Code** for an integrated experience. Here's how:

1. **Open VS Code**: Launch your Visual Studio Code editor.

2. **Install the Jupyter Extension**:

   * Go to the **Extensions** panel in VS Code.
   * Search for **Jupyter** and install the official **Jupyter** extension. This extension allows you to open and interact with Jupyter notebooks directly within VS Code.

3. **Create or Open a Jupyter Notebook**:

   * In VS Code, press **Ctrl + Shift + P** to open the **command palette**.
   * Search for and select **Jupyter: Create New Blank Notebook** to create a new notebook.
   * Alternatively, if you have an existing `.ipynb` file, you can open it directly in VS Code.

4. **Select Rust Kernel**:

   * Once the notebook is open, you’ll need to select the **Rust** kernel.
   * In the top-right corner of the notebook interface in VS Code, click on the **kernel** dropdown and select **Rust** from the list of available kernels.

5. **Write and Execute Rust Code**:

   * You can now start writing Rust code in the notebook cells. For example:

     ```rust
     println!("Hello, Rust in Jupyter!");
     ```

   * To run the code, press **Shift + Enter**, and the output will appear directly below the code cell.


### **Step 6: Install Rust-related Extensions in VS Code**

For a better Rust coding experience in VS Code, install these extensions:

1. **Rust (rls)**: Provides Rust syntax highlighting, autocompletion, and error checking.

   * Open the **Extensions** panel in VS Code.
   * Search for **Rust** and install the **Rust (rls)** extension.

2. **Jupyter**: If not already installed, this allows you to open and interact with Jupyter notebooks in VS Code.

   * Open the **Extensions** panel in VS Code.
   * Search for **Jupyter** and install the **Jupyter** extension.

### **Step 7: Running Rust in Jupyter Notebook via VS Code**

Now that you have everything set up, you can run Jupyter notebooks from within VS Code:

1. Open **VS Code** and launch a new or existing notebook.
2. Open the **command palette** (Ctrl + Shift + P), and search for `Jupyter: Create New Blank Notebook`.
3. Select **Rust** as the kernel for your notebook.
4. Start writing and running Rust code in your notebook.

### **Troubleshooting Tips:**

* If the Rust kernel does not show up in Jupyter or VS Code, ensure you’ve properly installed `evcxr_jupyter` and run the installation command again:

  ```bash
  evcxr_jupyter --install
  ```

* If you're using VS Code and the Rust kernel isn’t recognized, make sure the **Jupyter** extension is installed and that you're selecting the correct kernel in the VS Code interface.

* Restart Jupyter or your terminal if you experience any issues with kernel recognition.

---

### **Conclusion**

You have successfully set up Rust to run in Jupyter notebooks using `evcxr_jupyter` in VS Code. With this setup, you can experiment with Rust interactively, making it easier to learn and explore Rust code within Jupyter notebooks. This guide provides an integrated and powerful development environment for both Rust and Jupyter.


